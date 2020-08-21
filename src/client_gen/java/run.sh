set -ex

cd /output
java -Xmx250m -jar ./project_name-jar-with-dependencies.jar "$@"