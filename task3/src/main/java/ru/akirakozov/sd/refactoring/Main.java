package ru.akirakozov.sd.refactoring;

import org.eclipse.jetty.server.Server;
import org.eclipse.jetty.servlet.ServletContextHandler;
import org.eclipse.jetty.servlet.ServletHolder;
import ru.akirakozov.sd.refactoring.gateways.HTMLRenderer;
import ru.akirakozov.sd.refactoring.gateways.ProductSQLRepository;
import ru.akirakozov.sd.refactoring.gateways.Renderer;
import ru.akirakozov.sd.refactoring.service.ProductService;
import ru.akirakozov.sd.refactoring.servlet.AddProductServlet;
import ru.akirakozov.sd.refactoring.servlet.GetProductsServlet;
import ru.akirakozov.sd.refactoring.servlet.QueryServlet;

/**
 * @author akirakozov
 */
public class Main {
    static final String DATABASE_URL = "jdbc:sqlite:test.db";

    public static void main(String[] args) throws Exception {
        ProductSQLRepository productRepository = new ProductSQLRepository(DATABASE_URL);
        ProductService productService = new ProductService(productRepository);
        Renderer renderer = new HTMLRenderer();

        productRepository.initDatabase();

        Server server = new Server(8081);

        ServletContextHandler context = new ServletContextHandler(ServletContextHandler.SESSIONS);
        context.setContextPath("/");
        server.setHandler(context);

        context.addServlet(new ServletHolder(new AddProductServlet(productService, renderer)), "/add-product");
        context.addServlet(new ServletHolder(new GetProductsServlet(productService, renderer)),"/get-products");
        context.addServlet(new ServletHolder(new QueryServlet(productService, renderer)),"/query");

        server.start();
        server.join();
    }
}
