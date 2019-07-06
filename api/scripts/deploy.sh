# Shell script for deployment with travis ci
# Ref: https://haoliangyu.github.io/blog/2018/03/19/AWS-ECS-auto-deployment-with-Travis-CI/
#
# Set following environment variables
# - DOCKER_HUB_USER
# - DOCKER_HUB_PSW
# - IMAGE_REPO_URL
# - CLUSTER_NAME
# - SERVICE_NAME
# - AWS_ACCESS_KEY_ID
# - AWS_SECRET_ACCESS_KEY
# - AWS_DEFAULT_REGION

set -eu

# install AWS SDK
pip install --user awscli
export PATH=$PATH:$HOME/.local/bin

# install necessary dependency for ecs-deploy
#sudo add-apt-repository ppa:eugenesan/ppa
#sudo apt-get update
#sudo apt-get install jq -y

# install ecs-deploy
curl https://raw.githubusercontent.com/silinternational/ecs-deploy/master/ecs-deploy | \
  sudo tee /usr/bin/ecs-deploy
sudo chmod +x /usr/bin/ecs-deploy

docker login --username $DOCKER_HUB_USER --password $DOCKER_HUB_PSW

# build the docker image and push to an image repository
docker build -t aoj-icpc .
docker tag aoj-icpc $IMAGE_REPO_URL:latest
docker push $IMAGE_REPO_URL:latest

# update an AWS ECS service with the new image
ecs-deploy -c $CLUSTER_NAME -n $SERVICE_NAME -i $IMAGE_REPO_URL:latest

