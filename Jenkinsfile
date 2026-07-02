pipeline {
  agent any
  environment {
    IMAGE_NAME = "pharmacy_backend:${env.BUILD_NUMBER}"
    DOCKER_REGISTRY = ""
  }
  stages {
    stage('Checkout') {
      steps {
        echo 'Step: Checkout source from SCM'
        checkout scm
        echo 'Done: Checkout'
        echo 'Verificando checkout: rama, commit y listado de archivos'
        sh 'git rev-parse --abbrev-ref HEAD'
        sh 'git rev-parse --short HEAD'
        sh 'git log -1 --oneline'
        sh 'ls -la'
      }
    }

    stage('Build Rust release') {
      steps {
        dir('pharmacy_backend') {
          echo 'Step: Building Rust release (cargo build --release)'
          sh 'cargo build --release'
          echo 'Done: Rust release built'
        }
      }
    }

    stage('Build Docker image (temporary Dockerfile)') {
      steps {
        echo 'Step: Creating temporary Dockerfile and building image'
        sh '''
set -e
echo "Creating temporary Dockerfile at pharmacy_backend/Dockerfile.ci"
cat > pharmacy_backend/Dockerfile.ci <<'DOCK'
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
docker build -t ${IMAGE_NAME} -f pharmacy_backend/Dockerfile.ci pharmacy_backend
echo "Built image ${IMAGE_NAME}"

echo "Removing temporary Dockerfile"
rm -f pharmacy_backend/Dockerfile.ci
'''
        echo 'Done: Docker image built'
      }
    }

    stage('Run docker-compose and validate backend') {
      steps {
        echo 'Step: Starting backend via docker-compose and validating health'
        sh '''
set -e
echo "Exporting IMAGE_NAME=${IMAGE_NAME} for docker-compose"
export IMAGE_NAME=${IMAGE_NAME}

echo "Starting backend service from pharmacy_backend/docker-compose.yml"
docker-compose -f pharmacy_backend/docker-compose.yml up -d backend

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
  docker-compose -f pharmacy_backend/docker-compose.yml ps
  docker-compose -f pharmacy_backend/docker-compose.yml logs backend --tail=200 || true
  docker-compose -f pharmacy_backend/docker-compose.yml down
  exit 1
fi

echo "Backend validated successfully"

echo "Tearing down docker-compose services"
docker-compose -f pharmacy_backend/docker-compose.yml down
'''
        echo 'Done: Backend validated and compose torn down'
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
      sh 'docker-compose -f pharmacy_backend/docker-compose.yml down || true'
    }
  }
}
