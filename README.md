## Overview

This is a simple POC that rebuilds a catalog from a given set of filtered operators for a given index

## Use Case

As the current redhat-operator-index has something like 203 operators, when we use a filtered set of operators, the index needs to reflect
only the filtered operators we have selected. This means we have to re-build the catalog (with the filtered operators), use the embedded opm 
binary to re-create the cache, then tar the contents of the layer (that contains the configs directory), obtain a sha256 has and update the 
manifest to reflect the new layer


## POC 

I used a simple approach - Occam's razor

- A scientific and philosophical rule that entities should not be multiplied unnecessarily (KISS)
- Worked with a v2 images for the POC


## Usage

Clone this repo

Ensure that you have the correct permissions set in the $XDG_RUNTIME_DIR/containers/auth.json file

Update a FilterConfig file - I used this as an example

```
kind: FilterConfiguration
apiVersion: mirror.openshift.io/v1alpha2
catalog: registry.redhat.io/redhat/redhat-operator-index:v4.13
packages:
  - name: aws-load-balancer-operator
  - name: serverless-operator
  - name: rhpam-kogito-operator
```

Execute the following command

```
cargo run -- --config filter-config.yaml
```

Console output  

```
cargo run -- --config filter-config.yaml 
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/rust-operator-catalog-rebuild --config filter-config.yaml`
 INFO   : rust-operator-catalog-rebuild filter-config.yaml 
 DEBUG  : [
    Operator {
        name: "aws-load-balancer-operator",
    },
    Operator {
        name: "serverless-operator",
    },
    Operator {
        name: "rhpam-kogito-operator",
    },
]
 DEBUG  : image reference "registry.redhat.io/redhat/redhat-operator-index:v4.13"
 INFO   : catalog index exists - no further processing required
 INFO   : cache exists - no further processing required
 INFO   : full path for directory 'configs' working-dir/redhat-operator-index/v4.13/cache/a1bc7b/configs 
 INFO   : full path for opm binary directory working-dir/redhat-operator-index/v4.13/cache/2b46cc/usr/bin/registry 
 INFO   :   from working-dir/redhat-operator-index/v4.13/cache/a1bc7b/configs/aws-load-balancer-operator 
 INFO   :   to working-dir/redhat-operator-index/v4.13/cache/tmp-catalog/configs/aws-load-balancer-operator 
 INFO   :   from working-dir/redhat-operator-index/v4.13/cache/a1bc7b/configs/serverless-operator 
 INFO   :   to working-dir/redhat-operator-index/v4.13/cache/tmp-catalog/configs/serverless-operator 
 INFO   :   from working-dir/redhat-operator-index/v4.13/cache/a1bc7b/configs/rhpam-kogito-operator 
 INFO   :   to working-dir/redhat-operator-index/v4.13/cache/tmp-catalog/configs/rhpam-kogito-operator 
 INFO   : hash c3c9bf0a038c16b4522ac0bfcc529c1e5a7eb9a7016f2672ada19519f0e26491 
 INFO   : temp-catalog directory removed 
```

Observe that a new manifest.json is created (there should be a backup of the old version)
also a file called *redhat-operator-index:v4.12-rebuild* should be located in the 
*working-dir/redhat-operator-index/v4.13/* directory


