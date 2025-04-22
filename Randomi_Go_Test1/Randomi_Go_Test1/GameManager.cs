using Microsoft.VisualBasic;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;

namespace Randomi_Go_Test1
{
    internal class GameManager
    {
        public static Dictionary<(string, string), int> Start()
        {
            Dictionary<(string, string), int> players = new();
            string opponentName = "Ordinateur";
            string playerName = Interaction.InputBox("Question?", "Joueur", "Entrez votre nom");
            Debug.WriteLine(playerName + " sera opposé à " + opponentName);
            players.Add(("Player", playerName), 100);
            players.Add(("Opponent", opponentName), 100);
            return players;
        }
    }
}
