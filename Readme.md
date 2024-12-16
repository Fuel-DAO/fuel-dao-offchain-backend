# Offchain Service

The **Offchain Service** handles car booking API calls by communicating with the backend canister and sending confirmation emails to users. This service runs independently in a containerized environment using Docker and Docker Compose.

---

## Features

- **Car Booking API**: Interfaces with the backend canister to handle booking requests.
- **Email Notifications**: Sends booking confirmation emails to users after successful bookings.
- **Containerized Deployment**: Easily deployable using Docker and Docker Compose.

---

## Prerequisites

Make sure you have the following installed:

- **Docker**: [Install Docker](https://docs.docker.com/get-docker/)
- **Docker Compose**: [Install Docker Compose](https://docs.docker.com/compose/install/)

---

## Set up .env
- **Update .env from .env.template**
- **Uncomment the last line from Dockerfile to run the server**
```bash
CMD ["/fueldao-offchain-server"]
```

## Docker run
```bash
docker compose up
```