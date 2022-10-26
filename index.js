const {
    beatmapNew,
    beatmapFromBytes,
    beatmapFromContent,
    beatmapFromPath,
    beatmapAr,
    beatmapCs,
    beatmapHp,
    beatmapOd,
    calculatorNew,
    calculatorMapAttrs,
    calculatorDifficulty,
    calculatorPerformance,
    calculatorStrains,
    calculatorMode,
    calculatorMods,
    calculatorAcc,
    calculatorGeki,
    calculatorKatu,
    calculatorN300,
    calculatorN100,
    calculatorN50,
    calculatorMisses,
    calculatorCombo,
    calculatorPassedObjects,
    calculatorClockRate,
} = require("./index.node");

class Beatmap {
    constructor(origin) {
        this.map = beatmapNew(origin);
    }

    fromPath(path) {
        beatmapFromPath.call(this.map, path);

        return this;
    }

    fromContent(content) {
        beatmapFromContent.call(this.map, content);

        return this;
    }

    fromBytes(bytes) {
        beatmapFromBytes.call(this.map, bytes);

        return this;
    }

    ar(ar) {
        beatmapAr.call(this.map, ar);

        return this;
    }

    cs(cs) {
        beatmapCs.call(this.map, cs);

        return this;
    }

    hp(hp) {
        beatmapHp.call(this.map, hp);

        return this;
    }

    od(od) {
        beatmapOd.call(this.map, od);

        return this;
    }
}

class Calculator {
    constructor(params) {
        this.calculator = calculatorNew(params);
    }

    mapAttributes(map) {
        return calculatorMapAttrs.call(this.calculator, map);
    }

    difficulty(map) {
        return calculatorDifficulty.call(this.calculator, map);
    }

    performance(map) {
        return calculatorPerformance.call(this.calculator, map);
    }

    strains(map) {
        return calculatorStrains.call(this.calculator, map);
    }

    mode(mode) {
        calculatorMode.call(this.calculator, mode);

        return this;
    }

    mods(mods) {
        calculatorMods.call(this.calculator, mods);

        return this;
    }

    acc(acc) {
        calculatorAcc.call(this.calculator, acc);

        return this;
    }

    nGeki(nGeki) {
        calculatorGeki.call(this.calculator, nGeki);

        return this;
    }

    nKatu(nKatu) {
        calculatorKatu.call(this.calculator, nKatu);

        return this;
    }

    n300(n300) {
        calculatorN300.call(this.calculator, n300);

        return this;
    }

    n100(n100) {
        calculatorN100.call(this.calculator, n100);

        return this;
    }

    n50(n50) {
        calculatorN50.call(this.calculator, n50);

        return this;
    }

    nMisses(nMisses) {
        calculatorMisses.call(this.calculator, nMisses);

        return this;
    }

    combo(combo) {
        calculatorCombo.call(this.calculator, combo);

        return this;
    }

    passedObjects(passedObjects) {
        calculatorPassedObjects.call(this.calculator, passedObjects);

        return this;
    }

    clockRate(clockRate) {
        calculatorClockRate.call(this.calculator, clockRate);

        return this;
    }
}

module.exports = { Beatmap, Calculator };
