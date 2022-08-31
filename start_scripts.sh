export GOOGLE_APPLICATION_CREDENTIALS="service-account.json";

KILL_SCRIPT="kill "
#RUST_LOG=get_contest_goals=info ./target/release/get_contest_goals > get_contest_goals.log 2>&1 &
#KILL_SCRIPT="$KILL_SCRIPT $!"
#echo "Started gettings contest goals"
#
#RUST_LOG=get_dogs=info ./target/release/get_dogs > get_dogs.log 2>&1 &
#KILL_SCRIPT="$KILL_SCRIPT $!"
#echo "Started getting dogs"
#
#RUST_LOG=upload_files=info ./target/release/upload_files > upload_files.log 2>&1 &
#KILL_SCRIPT="$KILL_SCRIPT $!"
#echo "Started upload files to google cloud storage"

RUST_LOG=api=info ./target/release/api > api.log 2>&1 &
KILL_SCRIPT="$KILL_SCRIPT $!"
echo "Started the web server"

echo $KILL_SCRIPT > kill_scripts.sh
chmod +x kill_scripts.sh
