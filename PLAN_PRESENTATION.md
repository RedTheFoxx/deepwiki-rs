# Plan de présentation — Litho (deepwiki-rs)

> **Document destiné à un générateur de présentation PowerPoint.**
> Public cible : généraliste, peu technique (managers, chefs de projet, équipes produit, curieux de l'IA). Quelques termes de jargon sont admis s'ils sont expliqués (LLM, prompt, pipeline, C4).
> Ton souhaité : pédagogique, concret, avec des analogies. Environ 20 diapositives.
> Les fichiers sources cités en notes proviennent du dépôt `deepwiki-rs` ; les prompts complets sont dans `PROMPTS.md`.

---

## Partie 1 — Le problème et la promesse (diapos 1 à 4)

### Diapo 1 — Titre

- **Titre :** Litho — L'IA qui documente vos logiciels à votre place
- **Sous-titre :** Comment transformer automatiquement du code source en documentation d'architecture claire et à jour
- **Visuel :** logo/bannière Litho, ambiance « avant/après » (code brouillon → wiki propre)

### Diapo 2 — Le problème : la documentation qui ment

- Message clé : dans presque tous les projets logiciels, la documentation est **obsolète, incomplète ou inexistante**.
- Points d'appui :
  - Le code change tous les jours, la documentation, elle, est mise à jour « quand on a le temps » (c'est-à-dire jamais).
  - Conséquences concrètes : intégration des nouveaux arrivants lente, décisions techniques mal comprises, dépendance aux « sachants ».
- Analogie suggérée : un plan de bâtiment jamais mis à jour alors qu'on ajoute des étages chaque mois.

### Diapo 3 — La promesse de Litho

- **Litho lit votre code source et écrit lui-même la documentation d'architecture**, complète, illustrée de schémas, en plusieurs langues.
- Trois bénéfices à mettre en avant :
  1. Documentation **toujours synchronisée** avec le code (regénérable à chaque commit).
  2. Des **centaines d'heures économisées** en rédaction et maintenance.
  3. Un format **professionnel et standardisé** (modèle C4 — expliquer en une phrase : une façon de décrire un système du plus général au plus détaillé, comme un zoom progressif sur une carte).
- Détail rassurant : outil open source, écrit en Rust (rapide et fiable), fonctionne avec les principaux fournisseurs d'IA.

### Diapo 4 — Ce que Litho produit concrètement

- Montrer l'arborescence de sortie (simplifiée) :
  1. Vue d'ensemble du projet
  2. Vue d'architecture
  3. Flux de travail (workflows)
  4. Explorations approfondies par module
  5. Interfaces et points d'entrée
  6. Vue de la base de données (si applicable)
- Visuel : capture d'un document généré avec un diagramme Mermaid (schéma d'architecture auto-généré).

---

## Partie 2 — Comment ça marche : la vue d'ensemble (diapos 5 à 7)

### Diapo 5 — L'idée centrale : une équipe d'agents IA spécialisés

- Message clé : Litho n'envoie pas tout le code à une IA en une fois. Il orchestre **une équipe d'agents IA spécialisés**, chacun avec un rôle précis, comme un cabinet d'audit :
  - des **enquêteurs** qui explorent le code,
  - des **analystes** qui en tirent des conclusions,
  - des **rédacteurs** qui écrivent les documents finaux.
- Jargon à introduire ici (avec définition simple) :
  - **LLM** : le « moteur » d'intelligence artificielle (type ChatGPT).
  - **Prompt** : la consigne écrite qu'on donne à l'IA — chaque agent de Litho a la sienne, soigneusement rédigée.
  - **Pipeline** : une chaîne de travail où chaque étape nourrit la suivante.

### Diapo 6 — Le pipeline en 4 phases

- Schéma central (reprendre le flowchart du README, simplifié) :

```
Code source → ① Prétraitement → ② Recherche → ③ Rédaction → ④ Vérification → Documentation finale
```

1. **Prétraitement** : lire et résumer le code, repérer ce qui est important.
2. **Recherche** : comprendre le « pourquoi » — objectifs, architecture, processus métier.
3. **Rédaction** : composer les documents finaux, avec schémas.
4. **Vérification** : contrôler et réparer les diagrammes, produire un rapport.

- Point notable pour le public : chaque phase **mémorise ses résultats** dans une mémoire partagée, que les phases suivantes réutilisent (pas de redite, cohérence garantie).

### Diapo 7 — L'économie du système : cache et compression

- Deux mécanismes malins à vulgariser (jargon léger accepté) :
  - **Cache intelligent** : si une analyse a déjà été faite, on réutilise le résultat au lieu de payer un nouvel appel à l'IA → temps et coûts réduits (Litho affiche même les euros économisés).
  - **Compression de contexte** : quand un dossier est trop volumineux pour l'IA, un agent spécialisé le **résume d'abord** (prompt du `PromptCompressor`) pour ne garder que l'essentiel.
- Message : Litho est conçu pour être **économe** — important pour un usage en entreprise.

---

## Partie 3 — Phase 1 : le prétraitement — « lire et trier » (diapos 8 à 9)

### Diapo 8 — Les agents de lecture du code

- Analogie : avant d'écrire un rapport sur une entreprise, on commence par visiter les locaux et inventorier les services.
- Trois prompts appelés dans cette phase (à présenter comme des « fiches de mission ») :

| Agent | Sa mission (formulation grand public) | Prompt source |
|---|---|---|
| **Notation des répertoires** (DirectoryScorer) | « Note de 0 à 1 l'importance business de chaque dossier du projet » | `src/generator/preprocess/agents/directory_scoring.rs` |
| **Résumé des répertoires** (DirectorySummarizer) | « Résume chaque dossier et chaque fichier : rôle, responsabilités, fonctions clés » | `src/generator/preprocess/agents/directory_summary.rs` |
| **Analyse des relations** (RelationshipsAnalyze) | « Dessine la carte des dépendances : qui utilise quoi, quelles sont les couches du système » | `src/generator/preprocess/agents/relationships_analyze.rs` |

- Logique d'appel à expliquer : la notation sert à **prioriser** (on analyse en profondeur seulement ce qui compte), les résumés alimentent la mémoire partagée, l'analyse des relations opère en **deux temps si le projet est gros** (l'IA sélectionne d'abord elle-même les 5 à 20 dossiers les plus significatifs, puis les analyse).

### Diapo 9 — Astuce de conception : demander du JSON

- Point de vulgarisation intéressant : ces prompts exigent des réponses en **JSON strict** (format de données structuré) plutôt qu'en texte libre.
- Pourquoi : les réponses doivent être **relues par un programme**, pas par un humain. Litho vérifie chaque réponse et, en cas d'erreur, **renvoie le message d'erreur à l'IA** en lui demandant de corriger (prompts des extracteurs, `src/llm/client/ollama_extractor.rs` et `openai_compatible_extractor.rs`).
- Message clé : la fiabilité ne vient pas de la confiance en l'IA, mais des **garde-fous** autour d'elle.

---

## Partie 4 — Phase 2 : la recherche — « comprendre » (diapos 10 à 12)

### Diapo 10 — Les enquêteurs spécialisés

- Présenter les agents de recherche comme une équipe d'enquêteurs, chacun avec sa question :

| Agent | La question qu'il pose au code | Prompt source |
|---|---|---|
| **Contexte système** (SystemContextResearcher) | « À quoi sert ce projet ? Pour qui ? Quelles sont ses frontières ? » | `system_context_researcher.rs` |
| **Détecteur de domaines** (DomainModulesDetector) | « Quels sont les grands blocs fonctionnels et comment sont-ils reliés ? » | `domain_modules_detector.rs` |
| **Chercheur d'architecture** (ArchitectureResearcher) | « Quels choix techniques structurent le système ? » | `architecture_researcher.rs` |
| **Chercheur de workflows** (WorkflowResearcher) | « Quels sont les grands processus, étape par étape ? » | `workflow_researcher.rs` |
| **Analyste des frontières** (BoundaryAnalyzer) | « Par où entre-t-on dans le système ? (commandes, API, configuration) » | `boundary_analyzer.rs` |
| **Analyste base de données** (DatabaseOverviewAnalyzer) | « Quelles tables, procédures et relations dans la base ? » | `database_overview_analyzer.rs` |
| **Insights des modules clés** (KeyModulesInsight) | « Pour chaque grand bloc : comment est-il implémenté en détail ? » | `key_modules_insight.rs` |

- (Tous dans `src/generator/research/agents/`.)

### Diapo 11 — La logique d'enchaînement : chacun s'appuie sur les autres

- Schéma de dépendances simplifié à illustrer :

```
Contexte système
   ├──→ Détecteur de domaines ──→ Chercheur d'architecture
   │            │                Chercheur de workflows
   │            └──→ Insights des modules clés (un appel IA par domaine, en parallèle)
   └──→ Analyste des frontières
```

- Messages clés :
  - Le rapport de chaque enquêteur est **injecté dans le prompt** des suivants (section « Research Materials Reference » assemblée automatiquement par `step_forward_agent.rs`).
  - L'analyse des modules clés est **parallélisée** : un appel IA par domaine métier, exécutés simultanément — c'est là que Rust et l'orchestration font gagner du temps.
  - Certains agents ont le droit d'utiliser des **outils** (lire un fichier, explorer un dossier) pour vérifier par eux-mêmes : c'est le mode « agent » (ReAct). Si l'enquête tourne trop longtemps, un prompt de secours (`SummaryReasoner`, `src/llm/client/summary_reasoner.rs`) force une synthèse avec ce qui a été trouvé.

### Diapo 12 — Le savoir externe : brancher vos propres documents

- Fonctionnalité différenciante à valoriser : Litho peut **« monter » des documents existants** (PDF d'architecture, ADR, schémas SQL, specs d'API) comme sources de connaissance.
- Chaque catégorie de document est routée vers les agents concernés (ex. : les docs « database » nourrissent l'analyste base de données).
- Effet dans les prompts : chaque agent a une section « External Knowledge Integration » lui demandant de **confronter le code à la documentation officielle** et de signaler les écarts.
- Message clé : Litho ne se contente pas de décrire le code, il peut **détecter la dérive** entre ce qui était prévu et ce qui a été construit.

---

## Partie 5 — Phase 3 : la rédaction — « écrire » (diapos 13 à 15)

### Diapo 13 — Les rédacteurs

- Une fois l'enquête terminée, des agents **rédacteurs** transforment les rapports en documents lisibles :

| Rédacteur | Document produit | Prompt source |
|---|---|---|
| **OverviewEditor** | « Vue d'ensemble du projet » (niveau contexte C4) | `overview_editor.rs` |
| **ArchitectureEditor** | « Vue d'architecture » complète avec diagrammes | `architecture_editor.rs` |
| **WorkflowEditor** | « Flux de travail » avec schémas de processus | `workflow_editor.rs` |
| **KeyModuleInsightEditor** | Un document approfondi **par domaine métier** (en parallèle) | `key_modules_insight_editor.rs` |
| **BoundaryEditor** | Documentation des interfaces — **sans IA** : mise en forme directe des données | `boundary_editor.rs` |
| **DatabaseEditor** | Vue base de données avec diagrammes ERD — **sans IA** également | `database_editor.rs` |

- (Tous dans `src/generator/compose/agents/`.)
- Point pédagogique : quand la donnée est déjà structurée (frontières, base de données), Litho **n'appelle pas l'IA** — un simple programme met en forme. L'IA n'est utilisée que là où elle apporte de la valeur.

### Diapo 14 — L'art du prompt de rédaction

- Montrer (en extrait, vulgarisé) ce que contient un prompt de rédacteur :
  - Un **rôle** : « Tu es un expert en documentation d'architecture logicielle… »
  - Des **règles de sécurité pour les diagrammes** Mermaid (syntaxe stricte pour que les schémas s'affichent toujours).
  - Une **structure de document imposée** (plan type en sections numérotées).
  - Des **critères de qualité** : complétude, exactitude, lisibilité, utilité.
  - Une consigne anti-tics de langage : interdiction des phrases de transition creuses (« Maintenant que j'ai rassemblé les informations… »).
- Message clé : la qualité du résultat vient autant de **l'ingénierie des consignes** que du modèle d'IA lui-même.

### Diapo 15 — Le multilingue intégré

- Chaque prompt (système et utilisateur) reçoit automatiquement une **instruction de langue** (`src/i18n.rs`) : la même analyse peut produire une documentation en français, anglais, chinois, japonais, allemand… en changeant un simple paramètre.
- Exemple : `deepwiki-rs --target-language fr -p ./mon-projet`
- Détail intéressant : les diagrammes gardent des identifiants techniques en ASCII, seuls les libellés sont traduits (règle inscrite dans les prompts).

---

## Partie 6 — Phase 4 : vérification et écosystème (diapos 16 à 17)

### Diapo 16 — Le contrôle qualité : Mermaid Fixer

- Problème réel : les IA font parfois des erreurs de syntaxe dans les diagrammes → schémas qui ne s'affichent pas.
- Solution : un outil compagnon, **Mermaid Fixer**, qui :
  1. détecte les diagrammes cassés (validation réelle dans un moteur JavaScript),
  2. envoie le code fautif à l'IA avec un **prompt de réparation très cadré** (`mermaid-fixer/src/prompt.tpl`) exigeant un correctif en JSON avec explication de chaque changement,
  3. réinjecte le diagramme réparé dans la documentation.
- Message clé : boucle **détection → réparation → validation**, symbole de la philosophie Litho (l'IA proposée, la machine vérifie).

### Diapo 17 — Le rapport final et l'écosystème

- En fin d'exécution, Litho produit un **rapport de synthèse** : temps par phase, taux de réussite des analyses, économies réalisées grâce au cache (généré sans IA — `summary_generator.rs`).
- Écosystème :
  - **Litho Book** : lecteur web élégant pour consulter la documentation générée (avec recherche et chat IA).
  - **Mermaid Fixer** : réparateur de diagrammes (vu ci-dessus).
  - Intégration **CI/CD** : documentation regénérée à chaque commit.

---

## Partie 7 — Synthèse et ouverture (diapos 18 à 20)

### Diapo 18 — La carte complète des prompts (diapo de synthèse)

- Diapo visuelle récapitulant la vingtaine de prompts sur le schéma du pipeline :

```
① PRÉTRAITEMENT          ② RECHERCHE                    ③ RÉDACTION
  · Notation dossiers      · Contexte système             · Vue d'ensemble
  · Résumé dossiers        · Domaines métier              · Architecture
  · Relations (2 temps)    · Architecture                 · Workflows
                           · Workflows                    · Modules clés (×N)
  TRANSVERSAUX             · Frontières                   (frontières & BDD :
  · Compression contexte   · Base de données               sans IA)
  · Enveloppe JSON         · Modules clés (×N domaines)
  · Repli sur erreur                                    ④ VÉRIFICATION
  · Instruction de langue                                 · Réparation Mermaid
  · Synthèse ReAct
```

- Renvoi : l'inventaire détaillé avec le texte de chaque prompt est dans `PROMPTS.md`.

### Diapo 19 — Ce qu'il faut retenir

1. **Un problème universel** : la documentation logicielle est toujours en retard sur le code.
2. **Une réponse d'orchestration** : pas « une grosse IA », mais ~20 prompts spécialisés enchaînés en pipeline, chacun nourri des résultats des précédents.
3. **Des garde-fous partout** : formats JSON vérifiés, retry sur erreur, cache, compression, validation des diagrammes, génération sans IA quand c'est possible.
4. **Un résultat concret** : un wiki d'architecture professionnel, multilingue, illustré, regénérable en quelques minutes.

### Diapo 20 — Questions / Démo

- Proposition : démo en direct (`cargo install deepwiki-rs` puis `deepwiki-rs -p ./mon-projet -o ./docs`) ou vidéo de secours.
- Liens : dépôt GitHub `sopaco/deepwiki-rs`, documentation générée par Litho sur lui-même (méta !), `PROMPTS.md` pour les curieux.
- Question d'ouverture pour la salle : « Quels autres métiers de l'écrit technique pourraient être orchestrés de cette façon ? »

---

## Annexe pour le générateur de présentation

- **Charte visuelle suggérée :** couleurs par phase (bleu = prétraitement, violet = recherche, vert = rédaction, orange = vérification), reprenant le diagramme du README.
- **Diagrammes réutilisables depuis le README** (`README.md`) : pipeline 4 phases (flowchart), enchaînement des agents de recherche, séquence complète main → workflow → outlet.
- **Sources des contenus :**
  - Prompts et fichiers d'origine : `PROMPTS.md` (racine du dépôt).
  - Orchestration des phases : `src/generator/workflow.rs`.
  - Assemblage des prompts (opening / matériaux / closing + langue) : `src/generator/step_forward_agent.rs`.
- **Niveau de langage :** vulgarisé ; jargon autorisé et à définir à la première occurrence : LLM, prompt, pipeline, agent, C4, JSON, Mermaid, cache, CI/CD.
- **Durée cible :** 25–30 minutes + questions.
