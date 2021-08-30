// 路由机制：使用 正则表达式
use super::*;

use bytes::Bytes;
use regex::Regex;
use crate::util::ByteStr;


#[derive(Debug, Clone)]
pub(crate) struct PathPattern(Arc<Inner>);

#[derive(Debug)]
struct Inner {
    full_path_regex: Regex,
    capture_group_names: Box<[Bytes]>,
}

// 路由匹配： 基于 service
#[derive(Debug, Clone)]
pub struct Route<S, F> {
    pub(crate) pattern: PathPattern,
    pub(crate) svc: S,      // service
    pub(crate) fallback: F, // 回退机制，比如回退到 404，或者支持 SPA应用
}

// B 代表 Body
impl<S, F, B> Service<Request<B>> for Route<S, F>
where
    S: Service<Request<B>, Response = Response<BoxBody>> + Clone,
    F: Service<Request<B>, Response = Response<BoxBody>, Error = S::Error> + Clone,
    B: Send + Sync + 'static,
{
    type Response = Response<BoxBody>;
    type Error = S::Error;
    type Future = RouteFuture<S, F, B>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        if let Some(captures) = self.pattern.full_match(&req) {
            insert_url_params(&mut req, captures);
            let fut = self.svc.clone().oneshot(req);
            RouteFuture::a(fut, self.fallback.clone())
        } else {
            let fut = self.fallback.clone().oneshot(req);
            RouteFuture::b(fut)
        }
    }
}

#[derive(Debug)]
pub(crate) struct UrlParams(pub(crate) Vec<(ByteStr, ByteStr)>);

fn insert_url_params<B>(req: &mut Request<B>, params: Vec<(String, String)>) {
    let params = params
        .into_iter()
        .map(|(k, v)| (ByteStr::new(k), ByteStr::new(v)));

    if let Some(current) = req.extensions_mut().get_mut::<Option<UrlParams>>() {
        let mut current = current.take().unwrap();
        current.0.extend(params);
        req.extensions_mut().insert(Some(current));
    } else {
        req.extensions_mut()
            .insert(Some(UrlParams(params.collect())));
    }
}


impl PathPattern {
    pub(crate) fn new(pattern: &str) -> Self {
        assert!(pattern.starts_with('/'), "Route path must start with a `/`");

        let mut capture_group_names = Vec::new();

        let pattern = pattern
            .split('/')
            .map(|part| {
                if let Some(key) = part.strip_prefix(':') {
                    capture_group_names.push(Bytes::copy_from_slice(key.as_bytes()));

                    Cow::Owned(format!("(?P<{}>[^/]+)", key))
                } else {
                    Cow::Borrowed(part)
                }
            })
            .collect::<Vec<_>>()
            .join("/");

        let full_path_regex =
            Regex::new(&format!("^{}", pattern)).expect("invalid regex generated from route");

        Self(Arc::new(Inner {
            full_path_regex,
            capture_group_names: capture_group_names.into(),
        }))
    }

    pub(crate) fn full_match<B>(&self, req: &Request<B>) -> Option<Captures> {
        self.do_match(req).and_then(|match_| {
            if match_.full_match {
                Some(match_.captures)
            } else {
                None
            }
        })
    }

    pub(crate) fn prefix_match<'a, B>(&self, req: &'a Request<B>) -> Option<(&'a str, Captures)> {
        self.do_match(req)
            .map(|match_| (match_.matched, match_.captures))
    }

    fn do_match<'a, B>(&self, req: &'a Request<B>) -> Option<Match<'a>> {
        let path = req.uri().path();

        self.0.full_path_regex.captures(path).map(|captures| {
            let matched = captures.get(0).unwrap();
            let full_match = matched.as_str() == path;

            let captures = self
                .0
                .capture_group_names
                .iter()
                .map(|bytes| {
                    std::str::from_utf8(bytes)
                        .expect("bytes were created from str so is valid utf-8")
                })
                .filter_map(|name| captures.name(name).map(|value| (name, value.as_str())))
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect::<Vec<_>>();

            Match {
                captures,
                full_match,
                matched: matched.as_str(),
            }
        })
    }
}

struct Match<'a> {
    captures: Captures,
    // true if regex matched whole path, false if it only matched a prefix
    full_match: bool,
    matched: &'a str,
}

type Captures = Vec<(String, String)>;
