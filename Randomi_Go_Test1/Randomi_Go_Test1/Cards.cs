using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Randomi_Go_Test1
{
    public enum Element
    {
        Feu,
        Air,
        Terre,
        Eau
    }
    public enum Nature
    {
        Sort,
        Arme,
        Aliment
    }
    public enum Stars
    {
        Un,
        Deux,
        Trois,
        Quatre,
        Cinq
    }

    internal class Cards
    {
        public Guid Id { get; set; }
        public string Name { get; set; }
        public Enum Element { get; set; }
        public Enum Stars { get; set; }
        public Enum Nature { get;set; }
        public string Effect { get; set; }
        public int Attack {  get; set; }
        public int Heal { get; set; }
        public int Draw {  get; set; }
        public bool Dice { get; set; }



        public Cards(Guid id, string name, Enum element, Enum stars, Enum nature, string effect, int attack, int heal, int draw, bool dice)
        {
            Id = id;
            Name = name;
            Element = element;
            Stars = stars;
            Attack = attack;
            Heal = heal;
            Draw = draw;
            Dice = dice;

           
        }
    }
}
