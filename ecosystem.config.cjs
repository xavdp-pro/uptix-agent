module.exports = {
  apps: [
    {
      name: 'uptix-agent',
      script: './target/release/uptix-agent',
      instances: 1,
      autorestart: true,
      watch: false,
      env: {
        UPTIX_HUB_URL: 'http://localhost:3001',
        UPTIX_SERVER_NAME: 'production-server'
      }
    }
  ]
};
