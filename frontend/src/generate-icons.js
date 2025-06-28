// generate-icons.js
import sharp from 'sharp';
import { mkdirSync, writeFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Icône SVG (version sans texte pour l'app)
const iconSVG = `<svg width="1024" height="1024" viewBox="0 0 1024 1024" fill="none" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="bgGradient" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#1a1a2e;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#0a0a0a;stop-opacity:1" />
    </linearGradient>
    
    <linearGradient id="mainGradient" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#a78bfa;stop-opacity:1" />
      <stop offset="50%" style="stop-color:#9333ea;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#ec4899;stop-opacity:1" />
    </linearGradient>
    
    <linearGradient id="accentGradient" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" style="stop-color:#c084fc;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#f472b6;stop-opacity:1" />
    </linearGradient>
    
    <filter id="glow">
      <feGaussianBlur stdDeviation="20" result="coloredBlur"/>
      <feMerge>
        <feMergeNode in="coloredBlur"/>
        <feMergeNode in="SourceGraphic"/>
      </feMerge>
    </filter>
    
    <filter id="shadow">
      <feDropShadow dx="0" dy="20" stdDeviation="30" flood-opacity="0.3"/>
    </filter>
  </defs>
  
  <rect width="1024" height="1024" rx="240" fill="url(#bgGradient)"/>
  
  <circle cx="200" cy="200" r="300" fill="url(#mainGradient)" opacity="0.1"/>
  <circle cx="824" cy="824" r="400" fill="url(#accentGradient)" opacity="0.08"/>
  
  <g transform="translate(512, 512)">
    <circle cx="0" cy="0" r="340" fill="none" stroke="url(#mainGradient)" stroke-width="8" opacity="0.3"/>
    <circle cx="0" cy="0" r="320" fill="none" stroke="url(#mainGradient)" stroke-width="4" opacity="0.2"/>
    
    <g filter="url(#shadow)">
      <circle cx="0" cy="0" r="280" fill="url(#mainGradient)" opacity="0.15"/>
      
      <path d="M -160 20 L -60 120 L 180 -120" 
            stroke="url(#mainGradient)" 
            stroke-width="60" 
            stroke-linecap="round" 
            stroke-linejoin="round" 
            fill="none"
            filter="url(#glow)"/>
      
      <g transform="translate(-80, -100)">
        <rect x="-100" y="-60" width="200" height="260" rx="20" fill="white" opacity="0.1"/>
        <rect x="-80" y="-40" width="160" height="40" rx="8" fill="url(#mainGradient)" opacity="0.3"/>
        <rect x="-80" y="20" width="120" height="20" rx="6" fill="url(#mainGradient)" opacity="0.2"/>
        <rect x="-80" y="60" width="140" height="20" rx="6" fill="url(#mainGradient)" opacity="0.2"/>
        <rect x="-80" y="100" width="100" height="20" rx="6" fill="url(#mainGradient)" opacity="0.2"/>
      </g>
    </g>
    
    <circle cx="-200" cy="-180" r="8" fill="url(#accentGradient)" opacity="0.6"/>
    <circle cx="220" cy="-160" r="12" fill="url(#mainGradient)" opacity="0.5"/>
    <circle cx="200" cy="200" r="6" fill="url(#accentGradient)" opacity="0.7"/>
    <circle cx="-180" cy="190" r="10" fill="url(#mainGradient)" opacity="0.4"/>
  </g>
</svg>`;

// Tailles requises pour PWA
const sizes = [72, 96, 128, 144, 152, 192, 384, 512];

// Créer le dossier icons
mkdirSync('./static/icons', { recursive: true });

// Sauvegarder le SVG original
writeFileSync('./static/icons/icon.svg', iconSVG);
console.log('✅ SVG original sauvegardé');

// Convertir le SVG en buffer PNG haute résolution
const svgBuffer = Buffer.from(iconSVG);
const pngBuffer = await sharp(svgBuffer)
    .resize(1024, 1024)
    .png()
    .toBuffer();

// Sauvegarder l'icône 1024x1024
await sharp(pngBuffer)
    .toFile('./static/icons/icon-1024x1024.png');
console.log('✅ Icône 1024x1024 générée');

// Générer toutes les tailles pour PWA
for (const size of sizes) {
    await sharp(pngBuffer)
        .resize(size, size)
        .png()
        .toFile(`./static/icons/icon-${size}x${size}.png`);
    console.log(`✅ Icône ${size}x${size} générée`);
}

// Générer le favicon
await sharp(pngBuffer)
    .resize(32, 32)
    .png()
    .toFile('./static/favicon.png');
console.log('✅ Favicon généré');

// Générer une version Apple Touch Icon
await sharp(pngBuffer)
    .resize(180, 180)
    .png()
    .toFile('./static/apple-touch-icon.png');
console.log('✅ Apple Touch Icon généré');

// Créer un fichier README pour les icônes
const readme = `# Icônes EpiSign

## Fichiers générés
- **icon.svg** : Icône vectorielle originale
- **icon-1024x1024.png** : Icône haute résolution
- **icon-{size}x{size}.png** : Icônes PWA (${sizes.join(', ')})
- **favicon.png** : Favicon 32x32
- **apple-touch-icon.png** : Icône iOS 180x180

## Utilisation
Ces icônes sont automatiquement référencées dans:
- \`/static/manifest.json\` pour la PWA
- \`/src/app.html\` pour le favicon et l'Apple Touch Icon

## Régénération
Pour régénérer les icônes :
\`\`\`bash
npm run generate-icons
\`\`\`
`;

writeFileSync('./static/icons/README.md', readme);
console.log('\n✨ Toutes les icônes ont été générées avec succès !');
console.log('📁 Dossier: ./static/icons/');