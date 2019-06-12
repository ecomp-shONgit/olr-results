# Ergebnisse der Software-Entwicklung zur Layout Analyse
In den Ordnern liegen die Ergebnisse der Software-Arbeitsschritte als Bilder vor. Die Auswertung findet sich als Tabelle unten in diesem README.

# Inhalt
Ordner "ergXXXXXXXX": Ausgangsdaten

Unterordner "0_POI": Points of Interest Selektion

Unterordner "1_Rand_Traegermaterial": Blatt-/Buchrand Selektion

Unterordner "2_thresholding_freistellen": unterdrücktes Trägermaterial, Thresholding, getragenes Material

Unterordner "3_box_line_schmuck": Textblock Selektion, Zeilenauswertung, Buchschmuck JA/NEIN

Unterordner "4_altoxml": Daten im ALTO XML Format

# Stichproben

## Stichprobe 1: 
  - Umfang 128 Seiten
  - Kennzeichen: Entwicklungsgrundlage der Software
  - Quellen:
    - http....
    
## Stichprobe 2:
  - Umfang 214 Seiten
  - Kennzeichen: bisher vom Programm unbearbeitete Seiten
  - Quellen:
    - "boeckel_oratio_1589_00021": http://www.ocr-d.de/sites/all/GTDaten/wecker_kochbuch_1598.zip
    - "alberti_pictura_1540_0007", "alberti_pictura_1540_0008", "alberti_pictura_1540_0009": http://www.ocr-d.de/sites/all/GTDaten/alberti_pictura_1540.zip
    - "trota_mordtbrenner_1540_0011", "trota_mordtbrenner_1540_0013": http://www.ocr-d.de/sites/all/GTDaten/trota_mordtbrenner_1540.zip
    - "aventinus_grammatica_1515_0007", "aventinus_grammatica_1515_0008": http://www.ocr-d.de/sites/all/GTDaten/aventinus_grammatica_1515.zip
    - "witzstat_buchszbaum_1540_0018", "witzstat_buchszbaum_1540_0021": http://www.ocr-d.de/sites/all/GTDaten/witzstat_buchszbaum_1540.zip
    - "luther_babstum_1526_0010", "luther_babstum_1526_0011": http://www.ocr-d.de/sites/all/GTDaten/luther_babstum_1526.zip
    - "pinder_epiphanie_1506_0009", "pinder_epiphanie_1506_0010": http://www.ocr-d.de/sites/all/GTDaten/pinder_epiphanie_1506.zip
    - "sachs_drey_1553_0010", "sachs_drey_1553_0011": http://www.ocr-d.de/sites/all/GTDaten/sachs_drey_1553.zip
    - "boeschenstain_gedicht_1520_0002": http://www.ocr-d.de/sites/all/GTDaten/boeschenstain_gedicht_1520.zip
    - "heyden_paedono_1548_0007", "heyden_paedono_1548_0013": http://www.ocr-d.de/sites/all/GTDaten/heyden_paedono_1548.zip
    - "pistoris_regiment_1506_0009", "pistoris_regiment_1506_0010": http://www.ocr-d.de/sites/all/GTDaten/pistoris_regiment_1506.zip
    - "nn_historia_1500_0007", "nn_historia_1500_0009": http://www.ocr-d.de/sites/all/GTDaten/nn_historia_1500.zip
    - "oesterreicher_sachsen_1548_0007", "oesterreicher_sachsen_1548_0011": http://www.ocr-d.de/sites/all/GTDaten/oesterreicher_sachsen_1548.zip
    - "kistler_kraeuter_1500_0007", "kistler_kraeuter_1500_0021": http://www.ocr-d.de/sites/all/GTDaten/kistler_kraeuter_1500.zip
    - "rhegius_artzney_1529_0007", "rhegius_artzney_1529_0014": http://www.ocr-d.de/sites/all/GTDaten/rhegius_artzney_1529.zip
    - "petrarca_psalmi_1506_0010", "petrarca_psalmi_1506_0012": http://www.ocr-d.de/sites/all/GTDaten/petrarca_psalmi_1506.zip
    - "vespucci_insule_1506_0009", "vespucci_insule_1506_0019": http://www.ocr-d.de/sites/all/GTDaten/vespucci_insule_1506.zip
    - "osiander_predigt_1553_0007", "osiander_predigt_1553_0008": http://www.ocr-d.de/sites/all/GTDaten/osiander_predigt_1553.zip
    - "basilius_legendi_1515_0007", "basilius_legendi_1515_0008": http://www.ocr-d.de/sites/all/GTDaten/basilius_legendi_1515.zip
    - "brenz_abentmal_1550_0043", "brenz_abentmal_1550_0158": http://www.ocr-d.de/sites/all/GTDaten/brenz_abentmal_1550.zip
    - "aepinus_bekentnis_1548_0006", "aepinus_bekentnis_1548_0007": http://www.ocr-d.de/sites/all/GTDaten/aepinus_bekentnis_1548.zip
    - "karlstadt_sermon_1523_0006", "karlstadt_sermon_1523_0020": http://www.ocr-d.de/sites/all/GTDaten/karlstadt_sermon_1523.zip
    - "lucius_epithalamia_1585_00006", "lucius_epithalamia_1585_00014": http://www.ocr-d.de/sites/all/GTDaten/lucius_epithalamia_1585.zip
    - "boeckel_oratio_1589_00021": http://www.ocr-d.de/sites/all/GTDaten/boeckel_oratio_1589.zip
    - "dorn_uppedat_1507_00017", "dorn_uppedat_1507_00032": http://www.ocr-d.de/sites/all/GTDaten/dorn_uppedat_1507.zip

# Usage
In den "ergXXXXXXXX" Ordnern liegt eine index.html, diese verweist auf die UnterOrdner. Mit dem Rust Programm in jedem Ordner kann ein index.html erzeugt werden, um eine Website zur Ansicht der Ergebnisse zu erzeugen.

rustc towebsite.rs

./towebsite

# Auswertung

Stichprobe 1 (11.06.2019) http://ecomparatio.net/~khk/ergO11062019/index.html




