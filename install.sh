# Start postgres
service postgresql start

# Create database
su postgres -c "createdb dishes"
su postgres -c "psql -c \"ALTER USER postgres WITH PASSWORD 'password';\""

diesel setup
diesel migration run

./target/release/meals_api
