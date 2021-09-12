export GOOGLE_APPLICATION_CREDENTIALS="service-account.json";

./target/release/get_contest_goals > get_contest_goals.log &
echo "Started gettings contest goals"

./target/release/get_dogs > get_dogs.log &
echo "Started getting dogs"

./target/release/upload_files > upload_files.log &
echo "Started upload files to google cloud storage"

./target/release/api > api.log &
echo "Started the web server"