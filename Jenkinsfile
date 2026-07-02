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

    stage('Build Docker image from repository Dockerfile') {
      steps {
        echo 'Step: Building Docker image from repository Dockerfile'
        sh '''
set -e
cd "$WORKSPACE"
echo "Workspace: $WORKSPACE"
export IMAGE_NAME="${IMAGE_NAME:-pharmacy_backend:${BUILD_NUMBER:-latest}}"
echo "Building image ${IMAGE_NAME} using Dockerfile in repository"
docker build --pull -t ${IMAGE_NAME} .
echo "Built image ${IMAGE_NAME}"
'''
        echo 'Done: Docker image built from repository Dockerfile'
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

# Export required environment variables with defaults for CI/development
export DATABASE_URL="${DATABASE_URL:-postgres://postgres:admin@host.docker.internal:5432/postgres?options=--search_path=pharmacy}"
export PASSWORD_SALT="${PASSWORD_SALT:-ci-default-salt-value-change-in-production}"
export API_JWT_SECRET="${API_JWT_SECRET:-ci-default-jwt-secret-key-min-16-chars-for-prod}"
export API_JWT_SECRET_REFRESH="${API_JWT_SECRET_REFRESH:-ci-default-refresh-secret-min-16-chars-for-prod}"
export BACKEND_HOST_PORT="${BACKEND_HOST_PORT:-0}"

echo "Environment variables set:"
echo "  DATABASE_URL: $DATABASE_URL"
echo "  PASSWORD_SALT: ****"
echo "  API_JWT_SECRET: ****"
echo "  API_JWT_SECRET_REFRESH: ****"
echo "  BACKEND_HOST_PORT: ${BACKEND_HOST_PORT}"

echo "Cleaning up previous Docker Compose services and containers..."
docker compose -f docker-compose.yml down --remove-orphans || true
docker container prune -f --filter "label!=keep" || true

echo "Starting backend service from docker-compose.yml"
docker compose -f docker-compose.yml up -d backend

RESOLVED_PORT="$(docker compose -f docker-compose.yml port backend 8081 | awk -F: '{print $NF}' | tr -d '[:space:]')"
if [ -z "$RESOLVED_PORT" ]; then
  echo "Could not resolve mapped backend port from docker compose."
  docker compose -f docker-compose.yml ps
  docker compose -f docker-compose.yml logs backend --tail=200 || true
  exit 1
fi
echo "Backend mapped to localhost:${RESOLVED_PORT}"

echo "Waiting for the backend to respond on port ${RESOLVED_PORT}"
RETRIES=30
for i in $(seq 1 $RETRIES); do
  if curl -sS "http://localhost:${RESOLVED_PORT}/" >/dev/null 2>&1; then
    echo "Backend responded on port ${RESOLVED_PORT}"
    break
  fi
  echo "Waiting for backend (attempt $i/$RETRIES)..."
  sleep 2
done

echo "Checking final backend response"
if ! curl -sS "http://localhost:${RESOLVED_PORT}/" >/dev/null 2>&1; then
  echo "Backend did not respond on port ${RESOLVED_PORT}"
  docker compose -f docker-compose.yml ps
  docker compose -f docker-compose.yml logs backend --tail=200 || true
  exit 1
fi

echo "Backend validated successfully"
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
    success {
      echo 'Deploy completado: el contenedor backend queda levantado.'
    }
    failure {
      echo 'Pipeline fallido. No se ejecuta docker compose down para no bajar servicios.'
    }
  }
}
