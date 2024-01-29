FROM mongo:7.0.2

ENTRYPOINT mongod --port 27017 --replSet rs0 --bind_ip 0.0.0.0 & MONGOD_PID=$!; \
until (mongosh --host localhost:27017 --eval "rs.initiate({ _id: 'rs0', members: [{ _id: 0, host: 'localhost:27017' }] })"); do sleep 1; done; \
echo "Creating admin user"; \
until (mongosh mongodb://localhost:27017?replicaSet=rs0 --eval "(new Mongo('localhost:27017')).getDB('admin').createUser({ user: 'admin', pwd: 'password', roles: [ 'root' ] })"); do sleep 1; done; \
echo "REPLICA SET ONLINE"; wait $MONGOD_PID;

