
# MONGO_DATA=mongo_data
REDIS_DATA=redis_data
MONGO_SERVICE=mongo
REDIS_SERVICE=redis

function mongo_up() {
    if [ ! -d $MONGO_DATA ]; then
        echo "creating $MONGO_DATA"
        mkdir $MONGO_DATA
    fi
    podman run  --rm -d --name $MONGO_SERVICE -v $MONGO_DATA:/data/db  -p 27017:27017 mongo:latest

}

function redis_up() {
    if [ ! -d $REDIS_DATA ]; then
        echo "creating $REDIS_DATA"
        mkdir $REDIS_DATA
    fi
    podman run --rm -d --name $REDIS_SERVICE -p 6379:6379 redis
}

function reset_data() {
    if [ -d "$1" ]; then
        echo "Deleting $1"
        rm -Rf $1
    fi  
     echo "Creating $1"
    mkdir $1
}

function up() {
#    mongo_up
    redis_up
}

function down() {
    podman stop $MONGO_SERVICE > /dev/null 2>&1
    podman stop $REDIS_SERVICE > /dev/null 2>&1
}

function reset() {
#    reset_data $MONGO_DATA
    reset_data $REDIS_DATA
}

function clean() {
    podman rm $MONGO_SERVICE > /dev/null 2>&1
    podman rm $REDIS_SERVICE > /dev/null 2>&1

    if [ -d "$MONGO_DATA" ]; then
        echo "Deleting $MONGO_DATA"
        rm -Rf $1
    fi  

    if [ -d "$REDIS_DATA" ]; then
        echo "Deleting $REDIS_DATA"
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
