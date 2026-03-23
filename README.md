# About

RoomBooker is a simple system handing classroom reservations. The system allows to
- check available classrooms
- book selected classroom for a given time slot
- view all created bookings
- delete booking

# Running the application

## Requirements

- Linux based operating system
- docker with docker compose installed

## Building and running

1. Cloning the repository

    ```bash
    git clone https://github.com/Muciej/RoomBooker.git
    ```

2. Building the project locally

    **Note:** running instance of database is required for the app to work properly, the recommended way to obtain it is to run (within the RoomBooker directory):
    ```bash
    docker compose up -d db
    ```
    however one can provide own instance - in that case the environment variable `DATABASE_URL` with the URL of the running database instance must be set. Database must complain with the format used in the app - see `/diagrams` folder or check the `/migrations` directory to learn about it.

    Steps after database instance is up and working:
    ```bash
    cd RoomBooker
    cargo run
    ```
3. Bulding the project using docker
    ```bash
    cd RoomBooker
    docker compose up -d
    ```
    The app will be automatically exposed on default HTTP port. See `.env` and `docker-compose.yaml` to change those settings.

# Usage notes

## Administrator password
Be aware that the administrator of the application has the "super user" passowrd allowing him or her to delete every booking regardless of the password set by its creator. Choose admins wisely!


