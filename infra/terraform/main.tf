terraform {
  required_version = ">= 1.9.0"

  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.33"
    }
  }
}

provider "kubernetes" {
  config_path = "~/.kube/config"
}

resource "kubernetes_namespace" "hexen" {
  metadata {
    name = "hexen"
    labels = {
      "aln/tenant" = "googolswarm"
    }
  }
}
