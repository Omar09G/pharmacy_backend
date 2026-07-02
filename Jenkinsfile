pipeline {
  agent any
  environment {
    IMAGE_NAME = "pharmacy_backend:${env.BUILD_NUMBER}"
    DOCKER_REGISTRY = ""
  }
  stages {
    stage('Checkout') {
      steps {
        echo 'Step: Checkout source from Git URL'
        git url: 'https://github.com/Omar09G/pharmacy_backend.git', branch: 'master'
        echo 'Done: Checkout'
        echo 'Verificando checkout: rama, commit y listado de archivos'
        sh 'git rev-parse --abbrev-ref HEAD'
        sh 'git rev-parse --short HEAD'
        sh 'git log -1 --oneline'
        sh 'ls -la'
      }
    }

    stage('Validate Docker environment') {
      steps {
        echo 'Step: Validating Docker availability and socket permissions'
        sh '''
set -e
cd "$WORKSPACE"
echo "Workspace: $WORKSPACE"
if ! command -v docker >/dev/null 2>&1; then
  echo 'ERROR: Docker command is not installed or not available in PATH.'
  exit 1
fi

echo 'Docker version:'
docker --version

echo 'Docker Compose version:'
docker compose version

echo 'Checking Docker socket permissions:'
if [ -S /var/run/docker.sock ]; then
  ls -l /var/run/docker.sock
else
  echo 'ERROR: Docker socket /var/run/docker.sock does not exist.'
  exit 1
fi

echo 'Testing Docker daemon access:'
docker info >/dev/null 2>&1 || {
  echo 'ERROR: Cannot access Docker daemon. Permission denied or daemon not running.'
  docker info 2>&1 | sed 's/^/    /'
  exit 1
}

echo 'Docker environment is valid.'
'''
        echo 'Done: Docker environment validated'
      }
    }

    stage('Build Rust release') {
      steps {
        echo 'Step: Building Rust release using Rust container'
        sh '''
set -e
cd "$WORKSPACE"
echo "Workspace: $WORKSPACE"
if ! command -v docker >/dev/null 2>&1; then
  echo 'Docker is not installed on this agent.'
  exit 1
fi

docker run --rm -v "$PWD":/work -w /work rust:latest cargo build --release
'''
        echo 'Done: Rust release built'
      }
    }

    stage('Build Docker image (temporary Dockerfile)') {
      steps {
        echo 'Step: Creating temporary Dockerfile and building image'
        sh '''
set -e
echo "Creating temporary Dockerfile at Dockerfile.ci"
cat > Dockerfile.ci <<'DOCK'
FROM debian:stable-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
ENV PORT=8081
ENV SERVER_ADDR=0.0.0.0
ENV LOG_LEVEL=info
ENV RUST_LOG=info
COPY target/release/pharmacy_backend /app/pharmacy_backend
RUN mkdir -p /app/logs
EXPOSE 8081
CMD ["/app/pharmacy_backend"]
DOCK

echo "Building Docker image ${IMAGE_NAME} using temporary Dockerfile"
docker build -t ${IMAGE_NAME} -f Dockerfile.ci .
echo "Built image ${IMAGE_NAME}"

echo "Removing temporary Dockerfile"
rm -f Dockerfile.ci
'''
        echo 'Done: Docker image built'
      }
    }

    stage('Run docker compose and validate backend') {
      steps {
        echo 'Step: Starting backend via docker compose and validating health'
        sh '''
set -e
cd "$WORKSPACE"
echo "Workspace: $WORKSPACE"
export IMAGE_NAME="${IMAGE_NAME:-pharmacy_backend:${BUILD_NUMBER:-latest}}"
echo "Using IMAGE_NAME=${IMAGE_NAME} for docker-compose"

echo "Starting backend service from docker-compose.yml"
docker compose -f docker-compose.yml up -d backend

echo "Waiting for the backend to respond on port 8081"
RETRIES=30
for i in $(seq 1 $RETRIES); do
  if curl -sS http://localhost:8081/ >/dev/null 2>&1; then
    echo "Backend responded on port 8081"
    break
  fi
  echo "Waiting for backend (attempt $i/$RETRIES)..."
  sleep 2
done

echo "Checking final backend response"
if ! curl -sS http://localhost:8081/ >/dev/null 2>&1; then
  echo "Backend did not respond on port 8081"
  docker compose -f docker-compose.yml ps
  docker compose -f docker-compose.yml logs backend --tail=200 || true
  exit 1
fi

echo "Backend validated successfully"

echo "Leaving docker compose services running"
'''
        echo 'Done: Backend validated'
      }
    }

    stage('Push image (optional)') {
      when {
        expression { return env.DOCKER_REGISTRY?.trim() }
      }
      steps {
        sh '''
set -e
TARGET="${DOCKER_REGISTRY}/${IMAGE_NAME}"
docker tag ${IMAGE_NAME} ${TARGET}
docker push ${TARGET}
'''
      }
    }
  }

  post {
    always {
      echo 'Skipping teardown to leave Docker Compose services running.'
    }
  }
}
