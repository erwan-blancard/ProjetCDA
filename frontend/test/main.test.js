import * as GAME from "../game";
jest.spyOn(GAME, "initGame").mockImplementation(() => {});
import "../main";

describe("main.js", () => {
  it("doit appeler GAME.initGame une fois au chargement", () => {
    expect(GAME.initGame).toHaveBeenCalledTimes(1);
  });
});
