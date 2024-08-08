variable "registry" {
  default = "public.ecr.aws/cyndra"
}

variable "context" {
  default = "."
}

target "api" {
  dockerfile = "Containerfile"
  context = "${context}"
  tags = ["${registry}/api"]
  args = {
    crate = "cyndra-api"
  }
}

target "provisioner" {
  dockerfile = "Containerfile"
  context = "${context}"
  tags = ["${registry}/provisioner"]
  args = {
    crate = "cyndra-provisioner"
  }
}
