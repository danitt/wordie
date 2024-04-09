#!/bin/bash

###
# Simple script to delete saved words from Merriam Webster
###

strings=("indolence" "recapitulate" "phylogeny" "ontogeny" "ontology" "gratuitous" "dialectic" "Semitic" "secular" "Santeria" "lapidary" "topography" "modality" "scrivener" "proletarian" "spate" "malinger" "transmogrify" "dowager" "thaumaturgy" "perfunctory" "scholastic" "consonance")

for str in "${strings[@]}"; do
  curl 'https://www.merriam-webster.com/lapi/v1/wordlist/delete' \
  -H 'accept: */*' \
  -H 'accept-language: en-AU,en;q=0.6' \
  -H 'cache-control: no-cache' \
  -H 'content-type: application/x-www-form-urlencoded; charset=UTF-8' \
  -H 'cookie: at_check=true; remember_me=sha512a9fc7f337f1602f5f3cd687f4dae68716d643b383c675f81da332b70db817350a459d6c74279e8a135b636104c749c9e433cc08d6f1a992c2ef016e827e5de8e; MWCOMSESS=a309a880d7e51dbfad3f9aae11129ca4; mwl6tid=9837713590223408596; mboxEdgeCluster=35; user-data={%22is_logged_in%22:true%2C%22at_expires%22:1712634774%2C%22username%22:%22brave_teapot%22%2C%22given_name%22:%22%22%2C%22family_name%22:%22%22%2C%22email%22:%22dan@fate.org.au%22%2C%22username_status%22:%22%22%2C%22is_first_login%22:false%2C%22recentsCollDisabled%22:false%2C%22recentsTotalCollDisabled%22:false}; pvc=2; mbox=session#5d6293ab724140b6877a3bb6b57ed3bc#1712625864|PC#5d6293ab724140b6877a3bb6b57ed3bc.35_0#1775868804' \
  -H 'origin: https://www.merriam-webster.com' \
  --data-raw "word=${str}&type=dt"
done
