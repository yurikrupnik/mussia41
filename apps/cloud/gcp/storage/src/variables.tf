variable "project_id" {
  type = string
  default = "sdp-demo-388112" # Change to to your GCP project id
}

variable "repository_id" {
  type = string
  default = "container-repository-terraform"
}

variable "location" {
  type = string
  default = "europe-central2"
}

variable "repo_names" {
  type    = list(string)
  default = ["yurikrupnik/sdp-demo"] # Change to to your github org
}