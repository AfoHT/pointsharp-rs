import java.io.FileNotFoundException;
import java.io.PrintStream;
  class PointSharpSeed {
    public static void main(String args[]) throws FileNotFoundException {


        String activation_code = "539787";
        long value = 123456789L;

        int[] iIncrementalCounter_int = new int[8];

        iIncrementalCounter_int[7] = (int)(value & 0xFFL);
        iIncrementalCounter_int[6] = (int)(value >> 8 & 0xFFL);
        iIncrementalCounter_int[5] = (int)(value >> 16 & 0xFFL);
        iIncrementalCounter_int[4] = (int)(value >> 24 & 0xFFL);
        iIncrementalCounter_int[3] = (int)(value >> 32 & 0xFFL);
        iIncrementalCounter_int[2] = (int)(value >> 40 & 0xFFL);
        iIncrementalCounter_int[1] = (int)(value >> 48 & 0xFFL);
        iIncrementalCounter_int[0] = (int)(value >> 56 & 0xFFL);

        byte[] iSeed;
        iSeed = new byte[20];
        byte[] pad = new byte[]{-39, -99, -69, -12, -7, -105, 24, -17, -2, 37, -3, 0, 125, -4, -32, 107, -5, 80, 48, -91};
        System.arraycopy(pad, 0, iSeed, 0, 20);

        for (int i = 0; i < activation_code.length(); ++i) {
            iSeed[i] = (byte)activation_code.charAt(i);
        }
        System.out.println("activation_code");
        System.out.println(activation_code);

        System.out.println("Incremental Counter");
        StringBuilder out = new StringBuilder();
        for (int i : iIncrementalCounter_int) {
            out.append(String.format("%d ", i));
        }
        System.out.println(out.toString());

        System.out.println("iSeed");
        StringBuilder sb = new StringBuilder();
        for (byte b : iSeed) {
            sb.append(String.format("%02X", b));
        }
        System.out.println(sb.toString());

        PrintStream p=new PrintStream("javaoutfile.byte");
        p.write(iSeed, 0, 20);
    }
}
