using System;
using System.Collections.Generic;
using System.Linq;

namespace Randomi_Go_Test1
{
    internal class Deck
    {
        private static List<Cards> deck;

        static Deck()
        {
            InitializeDeck();
        }

        public static void InitializeDeck()
        {
            CreateDeckTest();
            ShuffleDeck();
        }

        private static void CreateDeckTest()
        {
            deck = new List<Cards>
            {
                new Cards(Guid.NewGuid(), "Boule de feu", Element.Feu, Stars.Deux, Nature.Sort, "Enlevez 4 points à votre adversaire", 4, 0, 0, false),
                new Cards(Guid.NewGuid(), "Fléchette", Element.Air, Stars.Un, Nature.Arme, "Enlevez 3 points à votre adversaire", 3, 0, 0, false),
                new Cards(Guid.NewGuid(), "Bulle", Element.Eau, Stars.Un, Nature.Sort, "Lancez un dé, enlevez le résultat à votre adversaire", 0, 0, 0, true),
                new Cards(Guid.NewGuid(), "Pioche", Element.Terre, Stars.Deux, Nature.Sort, "Enlevez 6 points à votre adversaire, piochez une carte", 6, 0, 1, false),
                new Cards(Guid.NewGuid(), "Airicot", Element.Air, Stars.Trois, Nature.Aliment, "Récupérez 15 points", 0, 15, 0, false),
                new Cards(Guid.NewGuid(), "Forage", Element.Terre, Stars.Cinq, Nature.Sort, "Enlevez 9 points à votre adversaire, piochez 3 cartes", 9, 0, 3, false),
                new Cards(Guid.NewGuid(), "Glace à l'eau", Element.Eau, Stars.Trois, Nature.Aliment, "Récupérez 7 points, piochez 2 cartes", 0, 7, 2, false),
                new Cards(Guid.NewGuid(), "Grain de poussière", Element.Terre, Stars.Un, Nature.Sort, "Enlevez 0 point à votre adversaire", 0, 0, 0, false),
                new Cards(Guid.NewGuid(), "Lance de flammes", Element.Feu, Stars.Cinq, Nature.Arme, "Enlevez 18 points à votre adversaire", 18, 0, 0, false),
                new Cards(Guid.NewGuid(), "Bille de feu", Element.Feu, Stars.Un, Nature.Sort, "Enlevez 1 point à votre adversaire", 1, 0, 0, false),
                new Cards(Guid.NewGuid(), "Petit sablé", Element.Terre, Stars.Deux, Nature.Aliment, "Récupérez 5 points", 0, 5, 0, false),
                new Cards(Guid.NewGuid(), "Pistolet à eau", Element.Eau, Stars.Deux, Nature.Arme, "Enlevez 7 points à votre adversaire", 7, 0, 0, false),
                new Cards(Guid.NewGuid(), "Pomme", Element.Terre, Stars.Deux, Nature.Aliment, "Récupérez 6 points", 0, 6, 0, false),
                new Cards(Guid.NewGuid(), "Zéphyr", Element.Air, Stars.Deux, Nature.Sort, "Enlevez 5 points à votre adversaire, piochez une carte", 5, 0, 1, false),
                
            };
        }

        public static List<Cards> GetDeck()
        {
            return new List<Cards>(deck);
        }

        public static void ShuffleDeck()
        {
            Random rng = new Random();
            int n = deck.Count;
            while (n > 1)
            {
                n--;
                int k = rng.Next(n + 1);
                (deck[n], deck[k]) = (deck[k], deck[n]); // Swap des cartes
            }
        }

        public static Cards DrawCard()
        {
            if (deck.Count != 0)
            {
                Cards drawnCard = deck[0];
                deck.RemoveAt(0);
                return drawnCard;
            }
            else
            {
                return null;


            }
        }
    }
}
