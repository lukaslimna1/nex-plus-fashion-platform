import { describe, expect, it } from "vitest";
import { normalizeFhcmMediaUrl, parseFhcmCalendar, parseFhcmCollectionPage, parseFhcmCollectionsIndex, parseFhcmMaisonCollections, selectFhcmSs27WomenswearCollection } from "./scrape-fhcm-collections.js";

describe("FHCM SS27 collection scraper", () => {
  it("discovers the French collection links and preserves the civil day", () => {
    const entries = parseFhcmCollectionsIndex(`
      <div class="edition-collections"><div class="list"><a href="/fr/collection/balmain-mode-feminine-printempsete-2027" data-img="/sites/default/files/styles/clist/public/coll/img/balmain.png?itok=abc"><h2>BALMAIN</h2><time datetime="2026-09-30T12:00:00Z">30 septembre 2026</time></a></div></div>
    `);
    expect(entries).toEqual([{
      name: "BALMAIN",
      localDate: "2026-09-30",
      collectionUrl: "https://www.fhcm.paris/fr/collection/balmain-mode-feminine-printempsete-2027",
      coverUrl: "https://www.fhcm.paris/sites/default/files/styles/clist/public/coll/img/balmain.png?itok=abc",
      discoveryMethod: "COLLECTIONS_INDEX"
    }]);
  });

  it("keeps lookbook order, exact source credit, cover fallback and verified embed", () => {
    const entry = {
      name: "DRIES VAN NOTEN",
      localDate: "2026-09-30",
      collectionUrl: "https://www.fhcm.paris/fr/collection/dries-van-noten-mode-feminine-printempsete-2027",
      coverUrl: "https://www.fhcm.paris/sites/default/files/styles/clist/public/coll/img/dries.png?itok=cover"
    };
    const record = parseFhcmCollectionPage(entry, `
      <article class="collection" data-name="DRIES VAN NOTEN - Mode Féminine Printemps/Été 2027">
        <div class="video-iframe"><iframe src="https://player.castr.com/d_abc123"></iframe></div>
        <div class="field-credits"><div class="item">@Lauchmetrics</div></div>
        <div class="field-gallery">
          <div class="item"><img class="image-style-lkt" src="/sites/default/files/styles/lkt/public/lme/hash/LOOK001.jpg?itok=one" /></div>
          <div class="item"><img class="image-style-lkt" src="/sites/default/files/styles/lkt/public/lme/hash/LOOK002.jpg?itok=two" /></div>
          <div class="item"><img class="image-style-lkt" src="/sites/default/files/styles/lkt/public/lme/hash/LOOK002.jpg?itok=two" /></div>
        </div>
      </article>
    `);
    expect(record.images).toHaveLength(2);
    expect(record.images.map((image) => image.sequenceNumber)).toEqual([1, 2]);
    expect(record.images[0]).toMatchObject({
      remoteUrl: "https://www.fhcm.paris/sites/default/files/styles/lkt/public/lme/hash/LOOK001.jpg?itok=one",
      providerAssetId: "lme/hash/LOOK001.jpg",
      sourcePageUrl: entry.collectionUrl,
      creditLine: "FHCM page credit: @Lauchmetrics",
      alternativeUrls: []
    });
    expect(record.coverAsset?.displayMode).toBe("THUMBNAIL_ONLY");
    expect(record.videos[0]).toMatchObject({
      provider: "Castr",
      providerAssetId: "d_abc123",
      embedUrl: "https://player.castr.com/d_abc123",
      videoType: "RUNWAY_COVERAGE",
      completeness: "UNKNOWN"
    });
  });

  it("deduplicates only an identical normalized file URL", () => {
    expect(normalizeFhcmMediaUrl("https://www.fhcm.paris/a/look.jpg?itok=one")).toBe("https://www.fhcm.paris/a/look.jpg");
    expect(normalizeFhcmMediaUrl("https://www.fhcm.paris/a/look-02.jpg?itok=one")).not.toBe(normalizeFhcmMediaUrl("https://www.fhcm.paris/a/look-03.jpg?itok=one"));
  });

  it("follows the French calendar to the exact Maison collection URL and avoids menswear or other seasons", () => {
    const calendar = parseFhcmCalendar(`
      <div class="calendar"><div class="day" data-day="20260930"><div class="cal-item"><a class="house-details" href="/fr/maison/dries-van-noten"><h3>DRIES VAN NOTEN</h3></a></div></div></div>
    `);
    expect(calendar).toEqual([{ name: "DRIES VAN NOTEN", localDate: "2026-09-30", maisonUrl: "https://www.fhcm.paris/fr/maison/dries-van-noten" }]);
    const links = parseFhcmMaisonCollections(`
      <div class="house-collections">
        <a href="/fr/collection/dries-van-noten-mode-masculine-printempsete-2027" data-img="/mens.jpg">DRIES VAN NOTEN - Mode Masculine Printemps/Été 2027</a>
        <a href="/fr/collection/dries-van-noten-mode-feminine-printempsete-2027" data-img="/womens.jpg">DRIES VAN NOTEN - Mode Féminine Printemps/Été 2027</a>
      </div>
    `, "https://www.fhcm.paris/fr/maison/dries-van-noten");
    expect(selectFhcmSs27WomenswearCollection(links)).toMatchObject({ collectionUrl: "https://www.fhcm.paris/fr/collection/dries-van-noten-mode-feminine-printempsete-2027", coverUrl: "https://www.fhcm.paris/womens.jpg" });
  });
});
