# DemocraChain

## Prerequisites of the Project

1. Create a Certificate Authority

```sh
openssl genrsa -out localhost-ca.key 2048
openssl req -x509 -new -nodes -key localhost-ca.key -sha256 -days 1024 -out localhost-ca.crt -subj "/C=US/ST=State/L=City/O=Organization/CN=localhost"
```

2. Create a certificate for localhost

```sh
openssl genrsa -out localhost.key 2048
openssl req -new -key localhost.key -out localhost.csr -subj "/C=US/ST=State/L=City/O=Organization/CN=localhost"
openssl x509 -req -in localhost.csr -CA localhost-ca.crt -CAkey localhost-ca.key -CAcreateserial -out localhost.crt -days 500 -sha256
```

## Run Project Locally

To run the Project locally:

1. Create a _.env_ File by running the Command:

   ```sh
   cp .env.example .env
   ```

2. Enter the Pinata API Key and Secret

3. Open a Terminal and run

   ```sh
   npx hardhat node
   ```

4. Copy the Account ID of the Any Account as the `OWNER_ADDRESS` in \*.env\_ File

5. Create Another Terminal and run

   ```sh
   npx hardhat run scripts/deploy.js --network localhost
   ```

6. Copy the Contract Address to the _.env_ File

7. Run the Command to start the Frontend

   ```sh
   1.npm i
   2.npm run dev
   ```
