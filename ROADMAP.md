# Strategisk Roadmap for Edge: Begrebs- & Informationsmodellering

Dette dokument samler langsigtede arkitektoniske visioner, konceptuelle overvejelser og fremtidige udviklingspotentialer for **Edge**, som kræver dybere domæneafklaring før de udmøntes i konkrete tasks.

---

## 🧭 1. Multi-Domæne Arkitektur, Kontekster & Begrebsgenbrug

### Baggrund & Udfordring
Ifølge Digitaliseringsstyrelsens FDA Modelregler (v2.1) bygger god modellering på princippet *"Genbrug frem for nyskabelse"*. I praksis eksisterer forretningsbegreber sjældent isoleret:
- Begrebet `Person` hører autoritativt hjemme i CPR / Identitets-domænet.
- Fagdomæner (f.eks. Uddannelse, Sundhed, Økonomi) skal kunne trække `Person` ind uden at genopfinde definitionen eller bryde det semantiske ophav.
- Samtidig kan et domæne have subdomæner eller afgrænsede kontekster (*Bounded Contexts*), hvor begrebet bærer lokale tillægsattributter eller roller (f.eks. `Studerende`, `Patient`, `Kunde`).

### Strategiske Modeller til Undersøgelse

#### Model A: Workspace & Multi-Model Federation (Anbefalet retning)
- **Koncept**: Et Edge Workspace kan rumme flere koblede modelprojekter (`.edge.json`).
- **Imports / Dependencies**: En model kan deklarere afhængigheder til autoritative kernemodeller (f.eks. `https://data.gov.dk/model/core/person.edge.json`).
- **Indlånte Begreber**: Når et begreb indlånes, refererer det direkte til kildemodellens URI og OID, og kan automatisk opdateres hvis kildemodellen revideres.

#### Model B: Domænehierarki med Navnerum (Namespaces)
- **Koncept**: En enkelt model kan definere et hierarki af domæner og subkontekster:
  ```text
  Identitet (Hoveddomæne)
    └── CPR-Registrering (Subkontekst)
          └── Person (Begreb)
  Uddannelse (Fagdomæne)
    └── Grundskole (Subkontekst)
          └── Elev (Specialisering af Identitet::Person)
  ```
- **Visninger (Views)**: Diagrammer kan filtreres på specifikke domæner/kontekster, så lærredet ikke oversvømmes af irrelevante noder.

---

## 🔒 2. Dataklassifikation & Sikkerhedsmærkning i Informationsmodellen

### Baggrund & Begrundelse
Begreber i en begrebsmodel repræsenterer virkelighedsfænomener og bærer ikke teknisk sikkerhedsklassifikation. Men når begreber operationaliseres til **UML-Klasser** og **Attributter** i informationsmodellen, opstår et direkte behov for datastyring og informationssikkerhed:

### Egenskaber til Informationsmodellen
1. **Fortrolighedsklassifikation (ISO 27001 / Fællesoffentlig standard)**:
   - `Åben` (Offentligt tilgængelige data)
   - `Intern` (Intern organisationsbrug)
   - `Fortrolig` (Fortrolige oplysninger, f.eks. forretningshemmeligheder)
   - `Strengt Fortrolig / Følsom` (CPR-numre, helbredsoplysninger)
2. **GDPR / Databeskyttelseskategori**:
   - `Almindelig personoplysning` (Navn, adresse)
   - `Særlig kategori / Følsom` (Helbred, race, religion, biometri)
   - `Straffedomme og lovovertrædelser`
   - `Ikke-personhenførbar data`
3. **Opbevaringsfrist & Slettekrav**:
   - Metadatafelt på attribut- eller klasseniveau til angivelse af slettefrister jf. arkivloven og GDPR.

---

## 🎨 3. Diagram Canvas Ergonomi & Avanceret Layout (xArchi Mønstre)

### Algoritmisk Auto-Layout
- **Hierarkisk Layout (Tree Left-to-Right og Tree Top-Down)**:
  - Implementering af Sugiyama-stil hierarkisk graf-layout for store diagrammer, så modellereren med ét klik kan rydde op i komplekse afhængigheder.
- **Undgå krydsende kanter (Edge Orthogonal Routing v2)**:
  - Yderligere forbedring af A*-stifinderen for ortogonale relationer til at minimere kantkryds og bevare lodrette/vandrette busser.

### Eksport & Dokumentationsgenerering
- **Vektor- & Billedeksport**:
  - Eksport af det aktive diagram til ren SVG og højopløselig PNG til indsættelse i rapporter og præsentationer.
- **FDA Bilag D & E Rapportgenerator**:
  - Ét-kliks eksport af hele modellen til en standardiseret HTML-, Markdown- eller PDF-rapport, der overholder Digitaliseringsstyrelsens krav til godkendelsesdokumenter.

---

## 🔄 4. Versionering, Historik & Model-Diff

- **Semantisk Model-Diff**:
  - Sammenlign to versioner af samme `.edge.json` og visualiser ændringer på diagrammet (f.eks. grønne noder for tilføjede begreber, røde for slettede, gule for ændrede definitioner).
- **Git-integration i UI**:
  - Integreret visning af Git branch og OID med mulighed for lokale checkpoints direkte fra applikationens statuslinje.
