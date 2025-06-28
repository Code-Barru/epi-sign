# Icônes EpiSign

## Fichiers générés
- **icon.svg** : Icône vectorielle originale
- **icon-1024x1024.png** : Icône haute résolution
- **icon-{size}x{size}.png** : Icônes PWA (72, 96, 128, 144, 152, 192, 384, 512)
- **favicon.png** : Favicon 32x32
- **apple-touch-icon.png** : Icône iOS 180x180

## Utilisation
Ces icônes sont automatiquement référencées dans:
- `/static/manifest.json` pour la PWA
- `/src/app.html` pour le favicon et l'Apple Touch Icon

## Régénération
Pour régénérer les icônes :
```bash
npm run generate-icons
```
