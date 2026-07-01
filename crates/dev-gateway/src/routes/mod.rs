#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Django,
    NextJs,
}

/// Boundary-aware matcher to resolve the target destination.
/// Resolves to Django if path matches prefix exactly or is a subpath (prefix + "/").
/// Otherwise, falls back to NextJs.
pub fn resolve_target(path: &str, prefix: &str) -> Target {
    if path == prefix || path.starts_with(&format!("{}/", prefix)) {
        Target::Django
    } else {
        Target::NextJs
    }
}

/// Rewrites the path by stripping the API prefix if Django is targeted and stripping is enabled.
pub fn rewrite_path<'a>(path: &'a str, target: Target, prefix: &str, strip: bool) -> &'a str {
    if target == Target::Django && strip {
        if path == prefix {
            ""
        } else if path.starts_with(&format!("{}/", prefix)) {
            &path[prefix.len()..]
        } else {
            path
        }
    } else {
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_resolution() {
        let prefix = "/api";

        // Django matches
        assert_eq!(resolve_target("/api/v1/todos", prefix), Target::Django);
        assert_eq!(resolve_target("/api", prefix), Target::Django);
        assert_eq!(resolve_target("/api/", prefix), Target::Django);

        // NextJs matches (adversarial boundary checks)
        assert_eq!(resolve_target("/apiary/v1/endpoints", prefix), Target::NextJs);
        assert_eq!(resolve_target("/api-websocket", prefix), Target::NextJs);
        assert_eq!(resolve_target("/", prefix), Target::NextJs);
        assert_eq!(resolve_target("/_next/static/chunks/main.js", prefix), Target::NextJs);
    }

    #[test]
    fn test_path_rewriting() {
        let prefix = "/api";

        // Strip prefix enabled
        assert_eq!(rewrite_path("/api/v1/todos", Target::Django, prefix, true), "/v1/todos");
        assert_eq!(rewrite_path("/api", Target::Django, prefix, true), "");
        assert_eq!(rewrite_path("/api/", Target::Django, prefix, true), "/");

        // Strip prefix disabled
        assert_eq!(rewrite_path("/api/v1/todos", Target::Django, prefix, false), "/api/v1/todos");
        assert_eq!(rewrite_path("/api", Target::Django, prefix, false), "/api");

        // Next.js target should not rewrite
        assert_eq!(rewrite_path("/api/v1/todos", Target::NextJs, prefix, true), "/api/v1/todos");
    }
}
