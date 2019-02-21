resource "aws_cloudfront_distribution" "blinging_love_distribution" {
  origin {
    domain_name = "${aws_s3_bucket.blinging_love_bucket.website_endpoint}"
    origin_id   = "${local.s3_origin_id}"

    custom_origin_config {
      http_port = 80
      https_port = 443
      origin_protocol_policy = "https-only"
      origin_ssl_protocols = ["TLSv1.2"]
    }
  }

  viewer_certificate {
    acm_certificate_arn = "${aws_acm_certificate.ssl_cert.arn}"
    ssl_support_method  = "sni-only"
  }

  restrictions {
    geo_restriction = {
      restriction_type = "whitelist"
      locations        = ["US", "CA"]
    }
  }

  default_cache_behavior {
    allowed_methods  = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "${local.s3_origin_id}"

    forwarded_values {
      cookies = {
        forward = "none"
      }

      query_string = true
    }

    viewer_protocol_policy = "redirect-to-https"
    min_ttl                = 0
    default_ttl            = 3600
    max_ttl                = 86400
  }

  enabled             = true
  is_ipv6_enabled     = true
  default_root_object = "index.html"
  aliases             = ["blinging.love"]
}
