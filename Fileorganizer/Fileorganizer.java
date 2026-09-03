import java.io.*;
import java.nio.file.*;
import java.util.*;

public class FileOrganizer {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        
        while (true) {
            System.out.println("\n=== File Organizer ===");
            System.out.println("1. Organize files");
            System.out.println("2. Exit");
            System.out.print("Choose: ");
            
            int choice = sc.nextInt();
            sc.nextLine();
            
            if (choice == 1) {
                System.out.print("Enter folder path: ");
                String folderPath = sc.nextLine();
                organizeFiles(folderPath);
            } else if (choice == 2) {
                System.out.println("Bye!");
                break;
            }
        }
    }
    
    static void organizeFiles(String folderPath) {
        File folder = new File(folderPath);
        
        if (!folder.exists()) {
            System.out.println("Folder not found!");
            return;
        }
        
        File[] files = folder.listFiles();
        if (files == null) return;
        
        for (File file : files) {
            if (file.isFile()) {
                String ext = getFileExtension(file.getName());
                String typeFolder = ext.isEmpty() ? "Other" : ext.toUpperCase();
                
                File typeDir = new File(folder, typeFolder);
                if (!typeDir.exists()) {
                    typeDir.mkdir();
                }
                
                try {
                    Files.move(file.toPath(), new File(typeDir, file.getName()).toPath());
                    System.out.println("✓ " + file.getName());
                } catch (Exception e) {
                    System.out.println("Error: " + file.getName());
                }
            }
        }
        System.out.println("✓ Done!");
    }
    
    static String getFileExtension(String filename) {
        int lastDot = filename.lastIndexOf('.');
        return lastDot > 0 ? filename.substring(lastDot + 1) : "";
    }
}
