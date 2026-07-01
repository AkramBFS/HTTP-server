use crate::config::RouteConfig;

/// Boundary-aware matcher to resolve the upstream destination.
/// If multiple routes match, the most specific path wins.
pub fn resolve_route<'a>(path: &str, routes: &'a [RouteConfig]) -> Option<&'a RouteConfig> {
    routes
        .iter()
        .filter(|route| route_matches(path, &route.path))
        .max_by_key(|route| route.path.len())
}

fn route_matches(path: &str, prefix: &str) -> bool {
    if prefix == "/" {
        return path.starts_with('/');
    }

    path == prefix || path.starts_with(&format!("{}/", prefix))
}

/// Rewrites the path by stripping a matched route prefix when configured.
pub fn rewrite_path<'a>(path: &'a str, prefix: &str, strip: bool) -> &'a str {
    if !strip || prefix == "/" {
        return path;
    }

    if path == prefix {
        ""
    } else if path.starts_with(&format!("{}/", prefix)) {
        &path[prefix.len()..]
    } else {
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_resolution_is_boundary_aware() {
        let routes = vec![
            RouteConfig {
                path: "/api".to_string(),
                target: "http://127.0.0.1:8000".to_string(),
                strip_prefix: true,
            },
            RouteConfig {
                path: "/".to_string(),
                target: "http://127.0.0.1:3000".to_string(),
                strip_prefix: false,
            },
        ];

        assert_eq!(
            resolve_route("/api/v1/todos", &routes).unwrap().path,
            "/api"
        );
        assert_eq!(resolve_route("/api", &routes).unwrap().path, "/api");
        assert_eq!(resolve_route("/api/", &routes).unwrap().path, "/api");

        assert_eq!(
            resolve_route("/apiary/v1/endpoints", &routes).unwrap().path,
            "/"
        );
        assert_eq!(resolve_route("/api-websocket", &routes).unwrap().path, "/");
        assert_eq!(resolve_route("/", &routes).unwrap().path, "/");
        assert_eq!(
            resolve_route("/_next/static/chunks/main.js", &routes)
                .unwrap()
                .path,
            "/"
        );
    }

    #[test]
    fn test_route_resolution_prefers_longest_match() {
        let routes = vec![
            RouteConfig {
                path: "/api".to_string(),
                target: "http://127.0.0.1:8000".to_string(),
                strip_prefix: true,
            },
            RouteConfig {
                path: "/api/admin".to_string(),
                target: "http://127.0.0.1:9000".to_string(),
                strip_prefix: true,
            },
        ];

        assert_eq!(
            resolve_route("/api/admin/users", &routes).unwrap().path,
            "/api/admin"
        );
    }

    #[test]
    fn test_path_rewriting() {
        let prefix = "/api";

        assert_eq!(rewrite_path("/api/v1/todos", prefix, true), "/v1/todos");
        assert_eq!(rewrite_path("/api", prefix, true), "");
        assert_eq!(rewrite_path("/api/", prefix, true), "/");

        assert_eq!(
            rewrite_path("/api/v1/todos", prefix, false),
            "/api/v1/todos"
        );
        assert_eq!(rewrite_path("/api", prefix, false), "/api");
        assert_eq!(rewrite_path("/api/v1/todos", "/", true), "/api/v1/todos");
    }
}
