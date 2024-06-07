bundle: {
//		_cluster: {
//			name:  string @timoni(runtime:string:TIMONI_CLUSTER_NAME)
//      group: string @timoni(runtime:string:TIMONI_CLUSTER_GROUP)
//      uid:   string @timoni(runtime:string:CLUSTER_UID)
//    }
    apiVersion: "v1alpha1"
    name:       "podinfo"
    instances: {
    	flux: {
				module: url: "oci://ghcr.io/stefanprodan/modules/flux-aio"
				namespace: "flux-system"
				values: {
						controllers: {
								helm: enabled:         true
								kustomize: enabled:    true
								notification: enabled: true
						}
						hostNetwork:     false
				  	securityProfile: "privileged"
				}
		  }
		  "prometheus": {
		  	module: url: "oci://ghcr.io/stefanprodan/modules/flux-helm-release"
			  namespace: "prometheus"
		 	  values: {
		 	  	repository: url: "https://prometheus-community.github.io/helm-charts"
					chart: {
						name:    "prometheus"
				 		version: "25.21.*"
					}
					helmValues: {
						installCRDs: true
					}
				}
		  }
		  "cert-manager": {
		  	module: url: "oci://ghcr.io/stefanprodan/modules/flux-helm-release"
			  namespace: "cert-manager"
		 	  values: {
		 	  	repository: url: "https://charts.jetstack.io"
					chart: {
						name:    "cert-manager"
				 		version: "1.x"
					}
					helmValues: {
						installCRDs: true
					}
				}
		  }
		  crossplane: {
		  	module: url: "oci://ghcr.io/stefanprodan/modules/flux-helm-release"
				namespace: "crossplane"
				values: {
					repository: url: "https://charts.crossplane.io/stable"
					chart: {
						name:    "crossplane"
					  version: "1.15.x"
					}
					helmValues: {
						installCRDs: true
					}
				}
			}
			komoplane: {
				module: url: "oci://ghcr.io/stefanprodan/modules/flux-helm-release"
				namespace: "komoplane"
				values: {
					repository: url: "https://helm-charts.komodor.io"
					chart: {
						name:    "komoplane"
				  	version: "0.1.x"
			 	  }
				  helmValues: {}
		 		}
		  }
//		  podinfo: {
//				module: url: "oci://ghcr.io/stefanprodan/modules/podinfo"
//				namespace: "apps"
//				values: {
//				  ui: message: "Hosted by \(_cluster.name) id \(_cluster.uid)"
//				  if _cluster.group == "staging" {
//						replicas: 1
//				  }
//				  if _cluster.group == "production" {
//						replicas: 2
//				  }
//			  }
//		  }
//		shit: {
//				module: {
//					url: "oci://me-west1-docker.pkg.dev/devops-386509/platform/crossplane-multi-cloud-storage"
//					version: "1.0.0"
//				}
//		}
//				 kyverno: {
//				 		module: url: "oci://ghcr.io/stefanprodan/manifests/kyverno"
//				 		module: version: "v1.8.0"
//				 		namespace: "kyverno"
//            values: {
//            }
//				 }



//        redis: {
//            module: {
//                url:     "oci://ghcr.io/stefanprodan/modules/redis"
//                version: "7.2.4"
//            }
//            namespace: "cache"
//            values: maxmemory: 256
//        }
//        podinfo: {
//            module: url:     "oci://ghcr.io/stefanprodan/modules/podinfo"
//            module: version: "6.5.4"
//            namespace: "podinfo"
//            values: caching: {
//                enabled:  true
//                redisURL: "tcp://redis:6379"
//            }
//        }
    }
}
