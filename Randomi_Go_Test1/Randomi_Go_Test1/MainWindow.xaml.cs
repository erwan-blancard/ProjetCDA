using System.Diagnostics;
using System.Text;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Media;
using System.Collections.Generic;
using System.Linq;
using System.Windows.Documents;
using System;
using System.Windows.Shapes;
using System.Windows.Media.Media3D;
using System.Diagnostics.Eventing.Reader;


namespace Randomi_Go_Test1
{
    public partial class MainWindow : Window
    {
        private bool isPlayerTurn;
        private int playerLife = 100;
        private int opponentLife = 100;
        public MainWindow()
        {
            InitializeComponent();
            StartGame();
            GetDeck();


        }

        public void StartGame()
        {
            Dictionary<(string, string), int> players = GameManager.Start();


            var playerEntry = players.FirstOrDefault(p => p.Key.Item1 == "Player");
            if (!playerEntry.Equals(default(KeyValuePair<(string, string), int>)))
            {
                string playerName = playerEntry.Key.Item2;
                int playerLife = playerEntry.Value;

                // Création du TextBlock pour le nom du joueur
                TextBlock textBlockPlayerName = new TextBlock
                {
                    Text = playerName,
                    FontSize = 20,
                    Foreground = Brushes.White,
                    FontWeight = FontWeights.Bold,
                    Margin = new Thickness(5),
                    TextAlignment = TextAlignment.Center
                };
                PlayerName.Children.Add(textBlockPlayerName);
                PlayerLifeText.Text = playerLife.ToString();
                // Création du TextBlock pour la vie du joueur
                //TextBlock textBlockPlayerLife = new TextBlock
                //{
                //    Text = playerLife.ToString(),
                //    FontSize = 20,
                //    Foreground = Brushes.White,
                //    FontWeight = FontWeights.Bold,
                //    Margin = new Thickness(5),
                //    TextAlignment = TextAlignment.Center
                //};
                //PlayerLife.Children.Add(textBlockPlayerLife);
            }

            // 🔹 Récupérer l’adversaire (nom et points de vie)
            var opponentEntry = players.FirstOrDefault(p => p.Key.Item1 == "Opponent");
            if (!opponentEntry.Equals(default(KeyValuePair<(string, string), int>)))
            {
                string opponentName = opponentEntry.Key.Item2;
                int opponentLife = opponentEntry.Value;

                // Création du TextBlock pour le nom de l’adversaire
                TextBlock textBlockOpponentName = new TextBlock
                {
                    Text = opponentName,
                    FontSize = 20,
                    Foreground = Brushes.White,
                    FontWeight = FontWeights.Bold,
                    Margin = new Thickness(5),
                    TextAlignment = TextAlignment.Center
                };
                OpponentName.Children.Add(textBlockOpponentName);

                // Création du TextBlock pour la vie de l’adversaire
                //TextBlock textBlockOpponentLife = new TextBlock
                //{
                //    Text = opponentLife.ToString(),
                //    FontSize = 20,
                //    Foreground = Brushes.White,
                //    FontWeight = FontWeights.Bold,
                //    Margin = new Thickness(5),
                //    TextAlignment = TextAlignment.Center
                //};
                //OpponentLife.Children.Add(textBlockOpponentLife);
            }

            UpdateDeckPileCount();
        }
        private void UpdateDeckPileCount()
        {
            DeckPile.Children.Clear();
            //afficher le nombre de cartes dans le deck
            int deckCount = Deck.GetDeck().Count;
            TextBlock textBlockDeck = new TextBlock
            {
                Text = "Nombre de carte(s) restante: " + deckCount,
                FontSize = 12,
                Foreground = Brushes.White,
                FontWeight = FontWeights.Bold,
                Margin = new Thickness(5),
                TextAlignment = TextAlignment.Center
            };
            DeckPile.Children.Add(textBlockDeck);
        }
        private void GetDeck()
        {
            Deck.InitializeDeck();
            List <Cards> deck = Deck.GetDeck();
            InitialDraw(deck);
            DiceStart();

        }

        private void ShuffleDeck(List<Cards> deck)
        {
            Random rng = new Random();
            deck = deck.OrderBy(c => rng.Next()).ToList();
            Debug.WriteLine(deck);
            DiceStart();
            InitialDraw(deck);
        }

        private void InitialDraw(List<Cards> deck)
        {
            if (deck.Count < 5)
            {
                MessageBox.Show("Pas assez de cartes dans le deck !");
                return;
            }

            // Distribue les cartes au joueur
            for (int i = 0; i < 5; i++)  // 5 cartes pour le joueur
            {
                Cards card = deck[0];  // Prend la première carte du deck
                deck.RemoveAt(0);  // Retire la carte du deck pour qu'elle ne soit pas redonnée

                Button btn = new Button
                {
                    Content = card.Name,
                    FontSize = 9,
                    Margin = new Thickness(3),
                    Background = Brushes.LightGray,
                    Width = 100,
                    Height = 25,
                };
                btn.Click += Card_Click;
                PlayerHand.Children.Add(btn);
            }

            // Distribue les cartes à l'adversaire
            for (int i = 0; i < 5 && deck.Count > 0; i++)  // 5 cartes pour l'adversaire
            {
                Cards card = deck[0];  // Prend la première carte du deck
                deck.RemoveAt(0);  // Retire la carte du deck

                System.Windows.Shapes.Rectangle rec = new System.Windows.Shapes.Rectangle()
                {
                    Width = 100,
                    Height = 25,
                    Fill = System.Windows.Media.Brushes.BlueViolet,
                    Stroke = System.Windows.Media.Brushes.Black,
                    StrokeThickness = 2,
                    Tag = card.Name
                };

                OpponentHand.Children.Add(rec);
                Debug.WriteLine("L'opposant dispose de la carte: " + card.Name);
            }
        }


        private void DiceStart()
        {
            Random rng = new Random();
            //MessageBox.Show("Lancer le dé");
            int playerInitialDice = rng.Next(1, 6);
            int opponentInitialDice = rng.Next(1, 6);
            if (playerInitialDice > opponentInitialDice) {
                //MessageBox.Show("Vous avez obtenu " + playerInitialDice + " et votre adversaire " + opponentInitialDice + ", vous commencez");
                InfoPanelText.Text = "Vous avez obtenu " + playerInitialDice + " et votre adversaire " + opponentInitialDice + ", vous commencez";
                Thread.Sleep(1000);

                isPlayerTurn = true;  // Le joueur commence
                PlayerTurn();
            }
            else if (playerInitialDice < opponentInitialDice)
            {
                //MessageBox.Show("Vous avez obtenu " + playerInitialDice + " et votre adversaire " + opponentInitialDice + ", l'ordinateur commence");
                InfoPanelText.Text = "Vous avez obtenu " + playerInitialDice + " et votre adversaire " + opponentInitialDice + ", l'ordinateur commence";
                Thread.Sleep(1000);

                isPlayerTurn = false; // L'ordinateur commence
                OpponentTurn(); // Si l'ordi commence, on appelle sa méthode de jeu
            }
            else if (playerInitialDice == opponentInitialDice)
            {
                //MessageBox.Show("Vous avez obtenu " + playerInitialDice + " et votre adversaire " + opponentInitialDice + ", relançons les dés pour départager");
                InfoPanelText.Text = "Vous avez obtenu " + playerInitialDice + " et votre adversaire " + opponentInitialDice + ", relançons les dés pour départager";
                Thread.Sleep(1000);
                DiceStart();
            }
        }
        private async void Card_Click(object sender, RoutedEventArgs e)
        {
            List<Cards> deck = new List<Cards>();
            if (!isPlayerTurn)
            {
                //MessageBox.Show("Ce n'est pas votre tour !");
                return;
            }
            
            Button clickedButton = sender as Button;
            if (clickedButton != null)
            {
                string cardName = clickedButton.Content.ToString();
                Debug.WriteLine($"Le joueur joue la carte : {cardName}");
                InfoPanelText.Text = $"Le joueur joue la carte : {cardName}";
                Cards c = Deck.GetDeck().FirstOrDefault(x => x.Name == cardName);
                PlayerLifeText.Text = (playerLife + c.Heal).ToString() ;
                if(playerLife > 100)
                {
                    playerLife = 100;
                    PlayerLifeText.Text = (playerLife + c.Heal).ToString();
                }
              
                OpponentLifeText.Text = (opponentLife - c.Attack).ToString();

                Debug.WriteLine("Le joueur enlève à l'adversaire : " + c.Attack);

                opponentLife = opponentLife - c.Attack;
                
                //OpponentLifeText.Text = opponentLife.ToString();
                await Task.Delay(1000);  // Donne le temps d'afficher le texte
                // Retirer la carte jouée
                PlayerHand.Children.Remove(clickedButton);

                // Passer au tour suivant
                NextTurn();
            }
        }

        private void NextTurn()
        {
            isPlayerTurn = !isPlayerTurn;
            PlayerTurn();

            if (!isPlayerTurn)
            {
                OpponentTurn(); // L'ordinateur joue
            }
            
        }
        private void PlayerTurn()
        {
            if (PlayerHand.Children.Count < 5)
            {
                Cards card = Deck.DrawCard(); // Pioche une carte
                Debug.WriteLine("Le joueur a pioché la carte: " + card.Name);   
                Button btn = new Button
                {
                    Content = card.Name,  // Affiche le nom
                    FontSize = 9,
                    Margin = new Thickness(3),
                    Background = Brushes.LightGray,
                    Width = 100,
                    Height = 25,

                };
                btn.Click += Card_Click;
                // Ajouter le bouton à la main du joueur
                PlayerHand.Children.Add(btn);
                UpdateDeckPileCount();

            }

        }

        private async void OpponentTurn()
        {
            if (OpponentHand.Children.Count<5)
            {
                Cards card = Deck.DrawCard(); // Pioche une carte
                System.Windows.Shapes.Rectangle rec = new System.Windows.Shapes.Rectangle()
                {
                    Width = 100,
                    Height = 25,
                    Fill = System.Windows.Media.Brushes.BlueViolet,
                    Stroke = System.Windows.Media.Brushes.Black,
                    StrokeThickness = 2,
                    Tag = card.Name
                };
                OpponentHand.Children.Add(rec);
                Debug.WriteLine("L'oppposant dispose de la carte: " + card.Name);
                UpdateDeckPileCount();
            }
            //MessageBox.Show("L'ordinateur réfléchit...");
            InfoPanelText.Text = "L'ordinateur réfléchit...";
            Thread.Sleep(1000);

            // Simuler un délai pour que le joueur voie l'ordi jouer
            System.Threading.Tasks.Task.Delay(1000).ContinueWith(_ =>
            {
                Dispatcher.Invoke(async () =>
                {
                    if (OpponentHand.Children.Count > 0)
                    {
                        Rectangle playedCard = OpponentHand.Children[0] as Rectangle;
                        if (playedCard != null)
                        {
                            string cardName = playedCard.Tag as string; // Récupérer le nom de la carte
                            //MessageBox.Show("L'ordinateur a joué la carte " + cardName + " !");
                            InfoPanelText.Text = "L'ordinateur a joué la carte " + cardName + " !";
                            Cards c = Deck.GetDeck().FirstOrDefault(x => x.Name == cardName);
                            OpponentLifeText.Text = (opponentLife + c.Heal).ToString();
                            if (opponentLife > 100)
                            {
                                opponentLife = 100;
                                OpponentLifeText.Text = (opponentLife + c.Heal).ToString();
                            }

                            PlayerLifeText.Text = (playerLife - c.Attack).ToString();

                            Debug.WriteLine($"L'ordinateur enlève au joueur :{c.Attack} points ");

                            opponentLife = opponentLife - c.Attack;

                        }
                        OpponentHand.Children.RemoveAt(0); // Supprimer la carte
                    }
                    await Task.Delay(1000);
                    InfoPanelText.Text = "A votre tour de jouer!";
                    NextTurn();
                });
            });
        }

        private void DrawCards(List<Cards> deck)
        {
            if (isPlayerTurn)
            {
                Cards card = Deck.DrawCard(); // Pioche une carte
                Button btn = new Button
                {
                    Content = card.Name,  // Affiche le nom
                    FontSize = 9,
                    Margin = new Thickness(3),
                    Background = Brushes.LightGray,
                    Width = 100,
                    Height = 25,

                };
                btn.Click += Card_Click;
                // Ajouter le bouton à la main du joueur
                PlayerHand.Children.Add(btn);


            }
            else if (!isPlayerTurn)
            {
                Cards card = Deck.DrawCard(); // Pioche une carte
                System.Windows.Shapes.Rectangle rec = new System.Windows.Shapes.Rectangle()
                {
                    Width = 100,
                    Height = 25,
                    Fill = System.Windows.Media.Brushes.BlueViolet,
                    Stroke = System.Windows.Media.Brushes.Black,
                    StrokeThickness = 2,
                    Tag = card.Name
                };

                OpponentHand.Children.Add(rec);
                Debug.WriteLine("L'oppposant dispose de la carte: " + card.Name);
            }
        }
            
               
        }


    }


