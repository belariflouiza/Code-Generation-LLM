public class CalculSomme {

    public int somme(int a, int b) {
        return a + b;
    }

    public static void main(String[] args) {
        CalculSomme calculateur = new CalculSomme();
        
        int a = 5;
        int b = 3;
        
        int resultat = calculateur.somme(a, b);
        
        System.out.println("La somme de " + a + " et " + b + " est égale à : " + resultat);
    }
}
