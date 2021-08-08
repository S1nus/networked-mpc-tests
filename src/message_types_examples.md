Player 0 generates 300 random doubles (x0, y0)
Player 0 generates a Paillier keypair.
For each of the 300 pairings of (x0, y0), Player 0 homomorphically-encrypts it with his Paillier's encryption key.

Player 0 starts his server and expects these two messages:

{
	"message_type": "intro",
	"player_number": 1
}

{
	"message_type": "intro",
	"player_number": 2
}

Once Player 0 has connections to Player 1 and Player 2, he sends each of them the 600 encrypted values, about 14 kilobytes each.

Player 1 generates his 300 random doubles, (x1, y1) and random number r_2.

for each pair of his pairs and Player 0s pairs, he computes t_1_0, and sends all those back to player 0

Player 2 generates his 300 random doubles, (x2, y2) and random numbers r_3_1 and r_3_2 for each one.
