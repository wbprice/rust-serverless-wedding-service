provider "aws" {
  region = "us-east-1"
}

module "frontend" {
  source = "./frontend"
}
