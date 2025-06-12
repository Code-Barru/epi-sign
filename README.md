## Epi-Sign

## Comment run le projet

### Pré-requis

1. Avoir Docker
2. Avoir un accès à [l'intra epitech](https://intra.epitech.eu)

### Lancer le projet

```shell
git clone git@github.com:Code-Barru/epi-sign.git
cd epi-sign
docker compose up -d
```

## Comment signer avec le projet

### Bypass l'anti-bot

Pour signer, il d'abord faut bypass le système anti-bot de l'intra d'epitech...

Après avoir lancé le container de la base de donnée :

Sur Windows :

```shell
./start-cookie-worker.bat
```

Sur Unix :

```shell
chmod u+x start-cookie-worker.sh
./start-cookie-worker.sh
```

Ensuite, les cookies sont valable 24h.

### Suite de call API à faire sur le backend

(voir la référence de l'api avec le swagger sur `http://localhost:3000/api/docs`)

Créer son compte et se login : `/api/auth/register` et `/api/auth/login`

Enregistrer son jwt : `/api/users/me/update-jwt`

Récupérer la liste des Ulids des User pour lesquels ont veut signer : `/api/users/`

Vérifier que le serveur a bien des cookies : `/api/sign/status`

Signer : `/api/sign`
