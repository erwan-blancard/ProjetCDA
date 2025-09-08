import * as THREE from 'three';
import gsap from 'gsap';

/**
 * Générateur d'effets visuels pour les cartes
 * Crée différents types d'effets selon l'action de la carte
 */
export class EffectGenerator {
    constructor(scene) {
        this.scene = scene;
    }

    /**
     * Crée un effet visuel pour une carte
     * @param {Object} card - La carte jouée
     * @param {Object} action - L'action de la carte
     * @returns {THREE.Object3D} L'effet créé
     */
    createCardEffect(card, action) {
        if (!card || !action) return null;

        // Position de départ (carte)
        const startPos = card.position ? card.position.clone() : new THREE.Vector3(0, 0, 0);
        startPos.y += 1; // Au-dessus de la carte

        // Associer les éléments aux types d'actions
        const elementMapping = this.getElementMapping(action.type);
        if (elementMapping) {
            action.element = elementMapping;
        }

        // Créer l'effet selon le type d'action
        let effect;
        switch (action.type) {
            case 'ATTACK':
                effect = this.createAttackEffect(startPos, action);
                break;
            case 'HEAL':
                effect = this.createHealEffect(startPos, action);
                break;
            case 'DRAW':
                effect = this.createDrawEffect(startPos, action);
                break;
            case 'DISCARD':
                effect = this.createDiscardEffect(startPos, action);
                break;
            case 'BOOST':
                effect = this.createBoostEffect(startPos, action);
                break;
            case 'STEAL':
                effect = this.createStealEffect(startPos, action);
                break;
            default:
                effect = this.createGenericEffect(startPos, action);
        }

        if (effect) {
            // Animation d'apparition
            this.animateEffectAppearance(effect);
        }

        return effect;
    }

    /**
     * Crée un effet d'attaque
     * @param {THREE.Vector3} position - Position de départ
     * @param {Object} action - Action d'attaque
     * @returns {THREE.Object3D} Effet d'attaque
     */
    createAttackEffect(position, action) {
        const group = new THREE.Group();
        group.position.copy(position);

        // Déterminer l'élément de la carte
        const element = action.element || 'fire'; // Par défaut feu
        const elementConfig = this.getElementConfig(element);

        // Particules selon l'élément
        const particleCount = Math.min(20 + action.amount * 2, 50);
        const particles = new THREE.BufferGeometry();
        const positions = new Float32Array(particleCount * 3);
        const colors = new Float32Array(particleCount * 3);

        for (let i = 0; i < particleCount; i++) {
            positions[i * 3] = (Math.random() - 0.5) * 2;
            positions[i * 3 + 1] = (Math.random() - 0.5) * 2;
            positions[i * 3 + 2] = (Math.random() - 0.5) * 2;

            // Couleur selon l'élément
            colors[i * 3] = elementConfig.color.r;     // R
            colors[i * 3 + 1] = elementConfig.color.g; // G
            colors[i * 3 + 2] = elementConfig.color.b; // B
        }

        particles.setAttribute('position', new THREE.BufferAttribute(positions, 3));
        particles.setAttribute('color', new THREE.BufferAttribute(colors, 3));

        const material = new THREE.PointsMaterial({
            size: elementConfig.particleSize,
            vertexColors: true,
            transparent: true,
            opacity: elementConfig.opacity
        });

        const points = new THREE.Points(particles, material);
        group.add(points);

        // Animation selon l'élément
        this.animateElementEffect(positions, particles, elementConfig, particleCount);

        return group;
    }

    /**
     * Crée un effet de soin
     * @param {THREE.Vector3} position - Position de départ
     * @param {Object} action - Action de soin
     * @returns {THREE.Object3D} Effet de soin
     */
    createHealEffect(position, action) {
        const group = new THREE.Group();
        group.position.copy(position);

        // Particules vertes pour le soin - commencent en haut et descendent
        const particleCount = Math.min(15 + action.amount, 30);
        const particles = new THREE.BufferGeometry();
        const positions = new Float32Array(particleCount * 3);
        const colors = new Float32Array(particleCount * 3);

        for (let i = 0; i < particleCount; i++) {
            // Commencer en haut (position.y + 2)
            positions[i * 3] = (Math.random() - 0.5) * 1.5;
            positions[i * 3 + 1] = 2 + Math.random() * 0.5; // Commence en haut
            positions[i * 3 + 2] = (Math.random() - 0.5) * 1.5;

            // Vert pour le soin
            colors[i * 3] = 0.2;     // R
            colors[i * 3 + 1] = 1.0; // G
            colors[i * 3 + 2] = 0.2; // B
        }

        particles.setAttribute('position', new THREE.BufferAttribute(positions, 3));
        particles.setAttribute('color', new THREE.BufferAttribute(colors, 3));

        const material = new THREE.PointsMaterial({
            size: 0.08,
            vertexColors: true,
            transparent: true,
            opacity: 0.7
        });

        const points = new THREE.Points(particles, material);
        group.add(points);

        // Animation de descente (particules qui tombent de haut en bas)
        gsap.to(positions, {
            duration: 1.5,
            ease: "power2.out",
            onUpdate: () => {
                particles.attributes.position.needsUpdate = true;
                // Faire descendre les particules
                for (let i = 0; i < particleCount; i++) {
                    positions[i * 3 + 1] -= 0.03; // Descente plus rapide
                    // Ajouter un léger mouvement horizontal
                    positions[i * 3] += (Math.random() - 0.5) * 0.01;
                    positions[i * 3 + 2] += (Math.random() - 0.5) * 0.01;
                }
            }
        });

        return group;
    }

    /**
     * Crée un effet de pioche
     * @param {THREE.Vector3} position - Position de départ
     * @param {Object} action - Action de pioche
     * @returns {THREE.Object3D} Effet de pioche
     */
    createDrawEffect(position, action) {
        const group = new THREE.Group();
        group.position.copy(position);

        // Utiliser l'élément glace pour la pioche
        const element = action.element || 'ice';
        const elementConfig = this.getElementConfig(element);

        // Effet de cristal de glace
        const particleCount = 15;
        const particles = new THREE.BufferGeometry();
        const positions = new Float32Array(particleCount * 3);
        const colors = new Float32Array(particleCount * 3);

        for (let i = 0; i < particleCount; i++) {
            const angle = (i / particleCount) * Math.PI * 2;
            const radius = 1.0;
            positions[i * 3] = Math.cos(angle) * radius;
            positions[i * 3 + 1] = Math.sin(angle) * radius;
            positions[i * 3 + 2] = 0;

            // Couleur de glace
            colors[i * 3] = elementConfig.color.r;
            colors[i * 3 + 1] = elementConfig.color.g;
            colors[i * 3 + 2] = elementConfig.color.b;
        }

        particles.setAttribute('position', new THREE.BufferAttribute(positions, 3));
        particles.setAttribute('color', new THREE.BufferAttribute(colors, 3));

        const material = new THREE.PointsMaterial({
            size: elementConfig.particleSize,
            vertexColors: true,
            transparent: true,
            opacity: elementConfig.opacity
        });

        const points = new THREE.Points(particles, material);
        group.add(points);

        // Animation de cristal
        this.animateCrystal(positions, particles, 1.5, particleCount);

        return group;
    }

    /**
     * Crée un effet de défausse
     * @param {THREE.Vector3} position - Position de départ
     * @param {Object} action - Action de défausse
     * @returns {THREE.Object3D} Effet de défausse
     */
    createDiscardEffect(position, action) {
        const group = new THREE.Group();
        group.position.copy(position);

        // Effet de spirale descendante
        const geometry = new THREE.ConeGeometry(0.2, 0.8, 6);
        const material = new THREE.MeshBasicMaterial({
            color: 0xff6600,
            transparent: true,
            opacity: 0.7
        });

        const cone = new THREE.Mesh(geometry, material);
        group.add(cone);

        // Animation de rotation et de descente
        gsap.timeline()
            .to(cone.rotation, { z: Math.PI * 2, duration: 0.8, ease: "power2.inOut" })
            .to(cone.position, { y: -1, duration: 0.8, ease: "power2.in" }, 0);

        return group;
    }

    /**
     * Crée un effet générique
     * @param {THREE.Vector3} position - Position de départ
     * @param {Object} action - Action générique
     * @returns {THREE.Object3D} Effet générique
     */
    createGenericEffect(position, action) {
        const group = new THREE.Group();
        group.position.copy(position);

        // Effet de lueur simple
        const geometry = new THREE.SphereGeometry(0.2, 8, 6);
        const material = new THREE.MeshBasicMaterial({
            color: 0xffffff,
            transparent: true,
            opacity: 0.5
        });

        const sphere = new THREE.Mesh(geometry, material);
        group.add(sphere);

        // Animation de pulsation
        gsap.to(sphere.scale, {
            x: 1.5,
            y: 1.5,
            z: 1.5,
            duration: 0.5,
            yoyo: true,
            repeat: 2,
            ease: "power2.inOut"
        });

        return group;
    }

    /**
     * Crée un effet de boost
     * @param {THREE.Vector3} position - Position de départ
     * @param {Object} action - Action de boost
     * @returns {THREE.Object3D} Effet de boost
     */
    createBoostEffect(position, action) {
        const group = new THREE.Group();
        group.position.copy(position);

        // Utiliser l'élément foudre pour le boost
        const element = action.element || 'lightning';
        const elementConfig = this.getElementConfig(element);

        // Effet de foudre
        const particleCount = 20;
        const particles = new THREE.BufferGeometry();
        const positions = new Float32Array(particleCount * 3);
        const colors = new Float32Array(particleCount * 3);

        for (let i = 0; i < particleCount; i++) {
            positions[i * 3] = (Math.random() - 0.5) * 3;
            positions[i * 3 + 1] = (Math.random() - 0.5) * 3;
            positions[i * 3 + 2] = (Math.random() - 0.5) * 3;

            // Couleur de foudre
            colors[i * 3] = elementConfig.color.r;
            colors[i * 3 + 1] = elementConfig.color.g;
            colors[i * 3 + 2] = elementConfig.color.b;
        }

        particles.setAttribute('position', new THREE.BufferAttribute(positions, 3));
        particles.setAttribute('color', new THREE.BufferAttribute(colors, 3));

        const material = new THREE.PointsMaterial({
            size: elementConfig.particleSize,
            vertexColors: true,
            transparent: true,
            opacity: elementConfig.opacity
        });

        const points = new THREE.Points(particles, material);
        group.add(points);

        // Animation électrique
        this.animateElectric(positions, particles, 2.0, particleCount);

        return group;
    }

    /**
     * Crée un effet de vol
     * @param {THREE.Vector3} position - Position de départ
     * @param {Object} action - Action de vol
     * @returns {THREE.Object3D} Effet de vol
     */
    createStealEffect(position, action) {
        const group = new THREE.Group();
        group.position.copy(position);

        // Utiliser l'élément ténèbres pour le vol
        const element = action.element || 'dark';
        const elementConfig = this.getElementConfig(element);

        // Effet de ténèbres
        const particleCount = 15;
        const particles = new THREE.BufferGeometry();
        const positions = new Float32Array(particleCount * 3);
        const colors = new Float32Array(particleCount * 3);

        for (let i = 0; i < particleCount; i++) {
            positions[i * 3] = (Math.random() - 0.5) * 2.5;
            positions[i * 3 + 1] = (Math.random() - 0.5) * 2.5;
            positions[i * 3 + 2] = (Math.random() - 0.5) * 2.5;

            // Couleur de ténèbres
            colors[i * 3] = elementConfig.color.r;
            colors[i * 3 + 1] = elementConfig.color.g;
            colors[i * 3 + 2] = elementConfig.color.b;
        }

        particles.setAttribute('position', new THREE.BufferAttribute(positions, 3));
        particles.setAttribute('color', new THREE.BufferAttribute(colors, 3));

        const material = new THREE.PointsMaterial({
            size: elementConfig.particleSize,
            vertexColors: true,
            transparent: true,
            opacity: elementConfig.opacity
        });

        const points = new THREE.Points(particles, material);
        group.add(points);

        // Animation de vide
        this.animateVoid(positions, particles, 1.5, particleCount);

        return group;
    }

    /**
     * Obtient l'élément associé à un type d'action
     * @param {string} actionType - Type d'action
     * @returns {string|null} Nom de l'élément
     */
    getElementMapping(actionType) {
        const mappings = {
            'DRAW': 'ice',        // Glace pour la pioche
            'STEAL': 'dark',      // Ténèbres pour le vol
            'BOOST': 'lightning'  // Foudre pour le boost
        };
        
        return mappings[actionType] || null;
    }

    /**
     * Obtient la configuration d'un élément
     * @param {string} element - Nom de l'élément
     * @returns {Object} Configuration de l'élément
     */
    getElementConfig(element) {
        const configs = {
            'fire': {
                color: { r: 1.0, g: 0.3, b: 0.0 },
                particleSize: 0.12,
                opacity: 0.9,
                animation: 'explosion',
                sound: 'attack_light'
            },
            'water': {
                color: { r: 0.2, g: 0.6, b: 1.0 },
                particleSize: 0.08,
                opacity: 0.7,
                animation: 'flow',
                sound: 'water_attack'
            },
            'earth': {
                color: { r: 0.6, g: 0.4, b: 0.2 },
                particleSize: 0.15,
                opacity: 0.8,
                animation: 'impact',
                sound: 'attack_heavy'
            },
            'air': {
                color: { r: 0.8, g: 0.9, b: 1.0 },
                particleSize: 0.06,
                opacity: 0.6,
                animation: 'swirl',
                sound: 'attack_light'
            },
            'lightning': {
                color: { r: 1.0, g: 1.0, b: 0.3 },
                particleSize: 0.1,
                opacity: 0.9,
                animation: 'electric',
                sound: 'attack_medium'
            },
            'ice': {
                color: { r: 0.7, g: 0.9, b: 1.0 },
                particleSize: 0.09,
                opacity: 0.8,
                animation: 'crystal',
                sound: 'attack_light'
            },
            'dark': {
                color: { r: 0.3, g: 0.1, b: 0.5 },
                particleSize: 0.11,
                opacity: 0.7,
                animation: 'void',
                sound: 'attack_heavy'
            }
        };
        
        return configs[element] || configs['fire'];
    }

    /**
     * Anime un effet selon son élément
     * @param {Float32Array} positions - Positions des particules
     * @param {THREE.BufferGeometry} particles - Géométrie des particules
     * @param {Object} elementConfig - Configuration de l'élément
     * @param {number} particleCount - Nombre de particules
     */
    animateElementEffect(positions, particles, elementConfig, particleCount) {
        const duration = 1.0 + Math.random() * 0.5; // 1.0 à 1.5 secondes
        
        switch (elementConfig.animation) {
            case 'explosion':
                this.animateExplosion(positions, particles, duration, particleCount);
                break;
            case 'flow':
                this.animateFlow(positions, particles, duration, particleCount);
                break;
            case 'impact':
                this.animateImpact(positions, particles, duration, particleCount);
                break;
            case 'swirl':
                this.animateSwirl(positions, particles, duration, particleCount);
                break;
            case 'electric':
                this.animateElectric(positions, particles, duration, particleCount);
                break;
            case 'crystal':
                this.animateCrystal(positions, particles, duration, particleCount);
                break;
            case 'void':
                this.animateVoid(positions, particles, duration, particleCount);
                break;
            default:
                this.animateExplosion(positions, particles, duration, particleCount);
        }
    }

    /**
     * Animation d'explosion (feu)
     */
    animateExplosion(positions, particles, duration, particleCount) {
        gsap.to(positions, {
            duration: duration,
            ease: "power2.out",
            onUpdate: () => {
                particles.attributes.position.needsUpdate = true;
                for (let i = 0; i < particleCount; i++) {
                    positions[i * 3] *= 1.1;
                    positions[i * 3 + 1] *= 1.1;
                    positions[i * 3 + 2] *= 1.1;
                }
            }
        });
    }

    /**
     * Animation de flux (eau)
     */
    animateFlow(positions, particles, duration, particleCount) {
        gsap.to(positions, {
            duration: duration,
            ease: "power2.out",
            onUpdate: () => {
                particles.attributes.position.needsUpdate = true;
                for (let i = 0; i < particleCount; i++) {
                    positions[i * 3] += 0.02;
                    positions[i * 3 + 1] += Math.sin(Date.now() * 0.001 + i) * 0.01;
                    positions[i * 3 + 2] += 0.01;
                }
            }
        });
    }

    /**
     * Animation d'impact (terre)
     */
    animateImpact(positions, particles, duration, particleCount) {
        gsap.to(positions, {
            duration: duration,
            ease: "power2.out",
            onUpdate: () => {
                particles.attributes.position.needsUpdate = true;
                for (let i = 0; i < particleCount; i++) {
                    positions[i * 3] *= 1.05;
                    positions[i * 3 + 1] *= 1.05;
                    positions[i * 3 + 2] *= 1.05;
                }
            }
        });
    }

    /**
     * Animation de tourbillon (air)
     */
    animateSwirl(positions, particles, duration, particleCount) {
        gsap.to(positions, {
            duration: duration,
            ease: "power2.out",
            onUpdate: () => {
                particles.attributes.position.needsUpdate = true;
                for (let i = 0; i < particleCount; i++) {
                    const angle = (i / particleCount) * Math.PI * 2 + Date.now() * 0.003;
                    const radius = 1.0 + Math.sin(Date.now() * 0.002) * 0.3;
                    positions[i * 3] = Math.cos(angle) * radius;
                    positions[i * 3 + 1] = Math.sin(angle) * radius;
                }
            }
        });
    }

    /**
     * Animation électrique (foudre)
     */
    animateElectric(positions, particles, duration, particleCount) {
        gsap.to(positions, {
            duration: duration,
            ease: "power2.out",
            onUpdate: () => {
                particles.attributes.position.needsUpdate = true;
                for (let i = 0; i < particleCount; i++) {
                    positions[i * 3] += (Math.random() - 0.5) * 0.1;
                    positions[i * 3 + 1] += (Math.random() - 0.5) * 0.1;
                    positions[i * 3 + 2] += (Math.random() - 0.5) * 0.1;
                }
            }
        });
    }

    /**
     * Animation de cristal (glace)
     */
    animateCrystal(positions, particles, duration, particleCount) {
        gsap.to(positions, {
            duration: duration,
            ease: "power2.out",
            onUpdate: () => {
                particles.attributes.position.needsUpdate = true;
                for (let i = 0; i < particleCount; i++) {
                    positions[i * 3] *= 1.02;
                    positions[i * 3 + 1] *= 1.02;
                    positions[i * 3 + 2] *= 1.02;
                }
            }
        });
    }

    /**
     * Animation de vide (ténèbres)
     */
    animateVoid(positions, particles, duration, particleCount) {
        gsap.to(positions, {
            duration: duration,
            ease: "power2.in",
            onUpdate: () => {
                particles.attributes.position.needsUpdate = true;
                for (let i = 0; i < particleCount; i++) {
                    positions[i * 3] *= 0.98;
                    positions[i * 3 + 1] *= 0.98;
                    positions[i * 3 + 2] *= 0.98;
                }
            }
        });
    }

    /**
     * Anime l'apparition d'un effet
     * @param {THREE.Object3D} effect - L'effet à animer
     */
    animateEffectAppearance(effect) {
        effect.scale.set(0, 0, 0);
        gsap.to(effect.scale, {
            x: 1,
            y: 1,
            z: 1,
            duration: 0.3,
            ease: "back.out(1.7)"
        });
    }

    /**
     * Crée un effet de shake screen pour les cartes sismiques
     * @param {Object} action - L'action de la carte
     * @returns {Object} Configuration de l'effet de shake
     */
    createScreenShakeEffect(action) {
        // Vérifier si c'est une carte sismique spécifique
        // IDs estimés basés sur la position dans cards.json
        const seismicCardIds = [122, 123, 89]; // Tremblement, Tremblottement, Séisme
        const isSeismic = seismicCardIds.includes(action.cardId);
        
        console.log(`🌍 Vérification shake: Card ID ${action.cardId}, Sismique: ${isSeismic}, IDs attendus: [${seismicCardIds.join(', ')}]`);
        
        if (!isSeismic) return null;
        
        // Intensité du shake selon la puissance de la carte
        const intensity = Math.min(action.amount / 10, 1.0); // 0.1 à 1.0
        const duration = 1.0 + (action.amount * 0.1); // 1.0 à 2.0 secondes
        
        return {
            intensity: intensity,
            duration: duration,
            frequency: 0.05, // Fréquence du shake (50ms)
            decay: 0.8 // Décroissance progressive
        };
    }

    /**
     * Applique l'effet de shake screen à la caméra
     * @param {THREE.Camera} camera - La caméra à secouer
     * @param {Object} shakeConfig - Configuration du shake
     */
    applyScreenShake(camera, shakeConfig) {
        if (!shakeConfig) return;
        
        const originalPosition = camera.position.clone();
        let currentIntensity = shakeConfig.intensity;
        const startTime = Date.now();
        
        const shakeInterval = setInterval(() => {
            const elapsed = (Date.now() - startTime) / 1000;
            
            // Calculer l'intensité décroissante
            const progress = elapsed / shakeConfig.duration;
            if (progress >= 1) {
                clearInterval(shakeInterval);
                camera.position.copy(originalPosition);
                return;
            }
            
            currentIntensity = shakeConfig.intensity * Math.pow(shakeConfig.decay, elapsed);
            
            // Appliquer le shake
            const shakeX = (Math.random() - 0.5) * currentIntensity * 0.5;
            const shakeY = (Math.random() - 0.5) * currentIntensity * 0.3;
            const shakeZ = (Math.random() - 0.5) * currentIntensity * 0.2;
            
            camera.position.set(
                originalPosition.x + shakeX,
                originalPosition.y + shakeY,
                originalPosition.z + shakeZ
            );
        }, shakeConfig.frequency * 1000);
    }

    /**
     * Crée un effet de teinte bleue progressive pour les cartes d'eau
     * @param {Object} action - L'action de la carte
     * @returns {Object} Configuration de l'effet de teinte
     */
    createWaterTintEffect(action) {
        // Vérifier si c'est une carte d'eau spécifique
        // IDs estimés basés sur la position dans cards.json
        const waterCardIds = [81, 62, 104]; // Raz de marée, Inondation, Vagues déferlantes
        const isWaterCard = waterCardIds.includes(action.cardId);
        
        console.log(`🌊 Vérification teinte: Card ID ${action.cardId}, Eau: ${isWaterCard}, IDs attendus: [${waterCardIds.join(', ')}]`);
        
        if (!isWaterCard) return null;
        
        // Intensité de la teinte selon la puissance de la carte
        const intensity = Math.min(action.amount / 15, 0.8); // 0.1 à 0.8
        const duration = 2.0 + (action.amount * 0.15); // 2.0 à 3.5 secondes
        
        return {
            intensity: intensity,
            duration: duration,
            color: { r: 0.2, g: 0.6, b: 1.0 }, // Bleu océan
            fadeInDuration: 0.5, // Temps de montée
            fadeOutDuration: 1.0 // Temps de descente
        };
    }

    /**
     * Applique l'effet de teinte bleue à l'écran
     * @param {Object} tintConfig - Configuration de la teinte
     */
    applyWaterTint(tintConfig) {
        if (!tintConfig) return;
        
        // Créer un overlay bleu semi-transparent
        const overlay = document.createElement('div');
        overlay.style.position = 'fixed';
        overlay.style.top = '0';
        overlay.style.left = '0';
        overlay.style.width = '100%';
        overlay.style.height = '100%';
        overlay.style.backgroundColor = `rgba(${tintConfig.color.r * 255}, ${tintConfig.color.g * 255}, ${tintConfig.color.b * 255}, 0)`;
        overlay.style.pointerEvents = 'none';
        overlay.style.zIndex = '9999';
        overlay.style.transition = `opacity ${tintConfig.fadeInDuration}s ease-in`;
        
        document.body.appendChild(overlay);
        
        // Animation de montée
        setTimeout(() => {
            overlay.style.backgroundColor = `rgba(${tintConfig.color.r * 255}, ${tintConfig.color.g * 255}, ${tintConfig.color.b * 255}, ${tintConfig.intensity})`;
        }, 10);
        
        // Animation de descente et suppression
        setTimeout(() => {
            overlay.style.transition = `opacity ${tintConfig.fadeOutDuration}s ease-out`;
            overlay.style.opacity = '0';
            
            setTimeout(() => {
                if (overlay.parentNode) {
                    overlay.parentNode.removeChild(overlay);
                }
            }, tintConfig.fadeOutDuration * 1000);
        }, tintConfig.duration * 1000);
    }
}
