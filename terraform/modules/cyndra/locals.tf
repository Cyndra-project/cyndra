data "aws_caller_identity" "current" {}

locals {
  account_id               = data.aws_caller_identity.current.account_id
  data_dir                 = "/opt/cyndra"
  docker_backend_image     = "public.ecr.aws/cyndra/api"
  docker_provisioner_image = "public.ecr.aws/cyndra/provisioner"
}
