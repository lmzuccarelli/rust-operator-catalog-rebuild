## Overview

This is a simple POC that rebuilds a catalog from a given set of filtered operators for a given index

## Use Case

As the current redhat-operator-index has something like 203 operators, wgen we use a filtered set of operators, the index needs to reflect
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

Observe that a new manifest.json is created (there should be a backup of the old version)
also a file called *redhat-operator-index:v4.12-rebuild* should be located in the 
*working-dir/redhat-operator-index/v4.13/* directory


