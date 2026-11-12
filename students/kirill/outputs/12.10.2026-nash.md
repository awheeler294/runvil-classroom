# The Prisoner's Dilemma

Two suspects face separate interrogations and each picks Silence or Betray once, without knowing the other's pick.
Betray is the dominant move for each, so both Betray and serve 2 years, although mutual Silence would give each only 1 year.

## Players

### Prisoner A

A suspect held in a separate cell who wants to minimize their own prison time.
They cannot talk to Prisoner B and do not know Prisoner B's pick.

| Game          | Description                                    |
| ------------- | ---------------------------------------------- |
| Interrogation | Suspect who picks Silence or Betray in secret. |

| Move    | Description                             |
| ------- | --------------------------------------- |
| Silence | Refuse to testify against the partner.  |
| Betray  | Testify against the partner for a deal. |

| Outcome    | Effect                                                     |
| ---------- | ---------------------------------------------------------- |
| Walk Free  | Betrays a Silent partner and serves 0 years.               |
| Short Time | Both stay Silent and each serves 1 year.                   |
| Hard Time  | Both Betray and each serves 2 years.                       |
| Sucker     | Stays Silent while the partner Betrays and serves 3 years. |

### Prisoner B

A suspect held in a separate cell who wants to minimize their own prison time.
They cannot talk to Prisoner A and do not know Prisoner A's pick.

| Game          | Description                                    |
| ------------- | ---------------------------------------------- |
| Interrogation | Suspect who picks Silence or Betray in secret. |

| Move    | Description                             |
| ------- | --------------------------------------- |
| Silence | Refuse to testify against the partner.  |
| Betray  | Testify against the partner for a deal. |

| Outcome    | Effect                                                     |
| ---------- | ---------------------------------------------------------- |
| Walk Free  | Betrays a Silent partner and serves 0 years.               |
| Short Time | Both stay Silent and each serves 1 year.                   |
| Hard Time  | Both Betray and each serves 2 years.                       |
| Sucker     | Stays Silent while the partner Betrays and serves 3 years. |

## Games

### Interrogation

- Trigger: Police arrest two suspects with evidence for only a minor charge.
- Players: Prisoner A, Prisoner B

1. Police hold the suspects in separate cells with no communication.
2. The prosecutor offers each suspect the same deal.
3. Each suspect picks Silence or Betray without knowing the other's pick.
4. The court applies the sentences for the pair of picks.

\newpage

## Payoff Analysis

| Game          | Equilibrium      | Welfare | Pareto-optimal | Gap | Status     |
| ------------- | ---------------- | ------- | -------------- | --- | ---------- |
| Interrogation | (Betray, Betray) | -4      | -2             | 2   | Deadweight |

### Interrogation

**Reduced payoff matrix:**

|                         | Prisoner B: Silence         | Prisoner B: Betray        |
| ----------------------- | --------------------------- | ------------------------- |
| **Prisoner A: Silence** | Short Time, Short Time (-2) | Sucker, Walk Free (-3)    |
| **Prisoner A: Betray**  | Walk Free, Sucker (-3)      | Hard Time, Hard Time (-4) |

- **Equilibrium:** (Betray, Betray) = -4
- **Pareto-optimal:** (Silence, Silence) = -2
- **Fix:** Add a Gang player with a Punish move and a final Interrogation step where the Gang imposes a 2-year penalty on each prisoner who Betrays; any penalty above 1 year makes Silence dominant and moves the equilibrium to (Silence, Silence) = -2.
- **Worse than deadweight:** (Betray, Betray) = -8 - the base game has no cell below -4, but under the Gang fix both prisoners serve Hard Time plus the penalty; reached if both prisoners doubt the Gang will Punish and Betray anyway.
