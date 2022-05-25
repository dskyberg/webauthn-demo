
MONGO_DATA=mongo_data
REDIS_DATA=redis_data


function reset_data() {
    if [ -d "$1" ]; then
        echo "Deleting $1"
        rm -Rf $1
    fi  
     echo "Creating $1"
    mkdir $1
}

function up() {
    if [ ! -d $MONGO_DATA ]; then
        echo "creating mongo"
        mkdir $MONGO_DATA
    fi
    podman run  --rm -d --name mongo -v $MONGO_DATA:/data/db  -p 27017:27017 mongo:latest

    if [ ! -d $REDIS_DATA ]; then
        echo creating redis
        mkdir $REDIS_DATA
    fi
    podman run  --rm -d --name redis -p 6379:6379 redis
}

function down() {
    podman stop mongo > /dev/null 2>&1
    podman stop redis > /dev/null 2>&1
}

function reset() {
    reset_data $MONGO_DATA
    reset_data $REDIS_DATA
}

function clean() {
    podman rm mongo > /dev/null 2>&1
    podman rm redis > /dev/null 2>&1

    if [ -d "$MONGO_DATA" ]; then
        echo "Deleting $MONGO_DATA"
        rm -Rf $1
    fi  

    if [ -d "$REDIT_DATA" ]; then
        echo "Deleting $REDIT_DATA"
        rm -Rf $1
    fi  

}

if [ "$1" = "reset" ]; then
    reset
elif [ $1 = "up" ]; then
    up
elif [ $1 = "down" ]; then
    down
elif [ $1 = "clean" ]; then
    clean
else
    echo "Unknwn command: $1"
fi
