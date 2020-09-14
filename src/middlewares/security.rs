use actix_web::middleware;

pub fn handler() -> middleware::DefaultHeaders
{
  // This is a NON exhaustive list of headers related to security
  middleware::DefaultHeaders::new()
    .header("Content-Security-Policy", // https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP
            "default-src 'self' localhost https://stackpath.bootstrapcdn.com https://fonts.googleapis.com https://fonts.gstatic.com;") 
    .header("X-DNS-Prefetch-Control", "off") // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-DNS-Prefetch-Control
    .header("Expect-CT", "max-age=86400") // https://developer.mozilla.org/en-US/docs/Web/Security/Certificate_Transparency
    .header("Strict-Transport-Security", "max-age=86400") // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Strict-Transport-Security
}
