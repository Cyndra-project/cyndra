locals {
  data_dir     = "/opt/cyndra"
  docker_image = "public.ecr.aws/cyndra/backend"
}

resource "random_string" "initial_key" {
  length  = 16
  special = false
  lower   = true
  number  = true
  upper   = true
}
