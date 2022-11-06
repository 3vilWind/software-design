package ru.akirakozov.sd.refactoring;

import ru.akirakozov.sd.refactoring.gateways.ProductRepository;
import ru.akirakozov.sd.refactoring.service.ProductService;

import java.lang.reflect.Array;
import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.SQLException;
import java.sql.Statement;

public class Utils {

    public static ProductService getProductServiceForTestDatabase() {
        ProductRepository productRepository = new ProductRepository("jdbc:sqlite:test.db");
        return new ProductService(productRepository);
    }

    public static void cleanTestDatabase() throws SQLException {
        try (Connection c = getTestDatabase()) {
            dropTable(c);
            createTable(c);
        }
    }

    public static Connection getTestDatabase() throws SQLException {
        return DriverManager.getConnection("jdbc:sqlite:test.db");
    }

    public static String[] getCommonHtmlStyling(String[] data) {
        String[] result = (String[]) Array.newInstance(String.class, data.length + 2);
        result[0] = "<html><body>";
        result[result.length - 1] = "</body></html>";
        System.arraycopy(data, 0, result, 1, data.length);
        return result;
    }

    private static void dropTable(Connection c) throws SQLException {
        String sql = "DROP TABLE IF EXISTS PRODUCT";
        Statement stmt = c.createStatement();

        stmt.executeUpdate(sql);
        stmt.close();
    }

    private static void createTable(Connection c) throws SQLException {
        String sql = "CREATE TABLE IF NOT EXISTS PRODUCT" +
                "(ID INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL," +
                " NAME           TEXT    NOT NULL, " +
                " PRICE          INT     NOT NULL)";
        Statement stmt = c.createStatement();

        stmt.executeUpdate(sql);
        stmt.close();
    }
}
