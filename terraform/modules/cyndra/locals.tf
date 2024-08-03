data "aws_caller_identity" "current" {}

locals {
  account_id               = data.aws_caller_identity.current.account_id
  data_dir                 = "/opt/cyndra"
  docker_backend_image     = "public.ecr.aws/cyndra/backend"
  docker_provisioner_image = "public.ecr.aws/cyndra/provisioner"
}

resource "random_string" "initial_key" {
  length  = 16
  special = false
  lower   = true
  number  = true
  upper   = true
}
