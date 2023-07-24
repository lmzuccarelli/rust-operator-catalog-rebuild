#! /bin/bash

set -eux pipefail

INDEX=working-dir/redhat-operator-index/v4.13

# set the index as current dir
pushd ${INDEX}

NEWCATALOG_CONFIGS=cache/tmp-catalog/configs
NEWCATALOG_DIR=cache/tmp-catalog
OPM_BINARY=cache/8e6990/usr/bin/registry/opm
TMP_TAR=cache/tmp-tar.tar

mkdir -p ${NEWCATALOG_CONFIGS}
# copy the desired/filtered catalogs
cp -r cache/40f21e/configs/3scale-operator ${NEWCATALOG_CONFIGS}
cp -r cache/40f21e/configs/advanced-cluster-management ${NEWCATALOG_CONFIGS}
cp -r cache/40f21e/configs/node-observability-operator ${NEWCATALOG_CONFIGS}
cp -r cache/40f21e/configs/aws-load-balancer-operator ${NEWCATALOG_CONFIGS}

# copy the var directory (just for good measure)
cp -r cache/40f21e/var cache/tmp-catalog/

# re-generate the cache
${OPM_BINARY} serve ${NEWCATALOG_CONFIGS} --cache-dir ${NEWCATALOG_DIR}/tmp/cache  --cache-only

# find the new sha256sum
tar -cvf ${TMP_TAR} ${NEWCATALOG_DIR} | sha256sum > sha-tmp.txt

NEWTAR=$(cat sha-tmp.txt | awk '{print $1}')

mv ${TMP_TAR} blobs/sha256/${NEWTAR}

# update manifest
cp manifest.json manifest-copy.json
sed -i "s/40f21e60a336cacba4271799bf3d0df447d5c8d7fd70a649c4b927c32e8c47a6/${NEWTAR}/g" manifest.json

# This step is not necessary, we have the blobs on disk 
# the containers mirror function uses the folder and manifest.json or index.json
# looks in the blobs/sha256 directory and then mirrors the image

# create new tar
tar -cvf redhat-operator-index.tar manifest.json blobs/ | sha256sum > digest.txt

# add sha256 digest
NEWDIGEST=$(cat digest.txt | awk '{print $1}')
mv redhat-operator-index.tar redhat-operator-index@${NEWDIGEST}

# cleanup
rm -rf sha-tmp.txt
rm -rf digest.txt

popd

mv ${INDEX}/redhat-operator-index@${NEWDIGEST} ./working-dir
