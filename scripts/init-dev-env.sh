# create k3d cluster
k3d cluster create $CLUSTER_NAME -p "${CLUSTER_PORT}:80@loadbalancer"
echo "export KUBECONFIG=\"$(k3d kubeconfig write gws)\"" >> ~/.bashrc

# install sqlx-cli
cargo install sqlx-cli

# create .env file
cp .env.example .env

# start postgres service
sudo service postgresql start

# create database
sqlx database reset --source ./api/migrations/ -y