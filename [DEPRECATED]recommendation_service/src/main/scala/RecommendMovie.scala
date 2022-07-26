// package com.scylladb
package sparkCassandra



import org.apache.log4j.{Level, Logger}
import org.apache.spark.rdd.RDD
import org.apache.spark.{SparkConf, SparkContext}
import org.apache.spark.mllib.recommendation.{ALS, MatrixFactorizationModel, Rating}
import org.apache.spark.sql.functions._
// import org.joda.time.DateTime
import com.datastax.spark.connector._
import org.apache.spark.sql.cassandra._
import org.apache.spark.sql.{Row, SparkSession}

object RecommendMovie  {
    
    def main(args: Array[String]): Unit = { 
        val sargs = Array("--USER", "200")
        
        val spark_configuration = new SparkConf(true)
            .setMaster("local")
            .set("spark.ui.showConsoleProgress", "false")
            .set("spark.cassandra.connection.host", "localhost")
            .set("spark.cassandra.connection.port", "9042")
            // .set("spark.cassandra.auth.username", "scylla")
            // .set("spark.cassandra.auth.password", "scylla")
            .setAppName("CollaborativeFilter");
        val spark_context = new SparkContext(spark_configuration);
        // println("🧑‍🔧 Setting up MasterNode " + spark_context  .master)
        //  Running Logger
        println("🛰️ Running Logger")
        Logger.getLogger("org").setLevel(Level.ERROR);
        Logger.getLogger("akka").setLevel(Level.ERROR);


        val input_id = sargs(1)
        val recommend_type = sargs(0)

        println("🏗️ Loading Data");
        val data_path = "/home/philip/rust-programming/fullstack/Netflix-Clone-WASM/recommendation_service/ml-100k/u.item";
        val model_path = "/home/philip/rust-programming/fullstack/Netflix-Clone-WASM/recommendation_service/ml-100k/ALSmodel";
        val checkpoint_path = "/home/philip/rust-programming/fullstack/Netflix-Clone-WASM/recommendation_service/ml-100k/checkpoint/";
        spark_context.setCheckpointDir(checkpoint_path)

        println("🐕 Filtering Data");
        val movie_title: RDD[(Int, String)] = prepareData(spark_context, data_path);
        movie_title.checkpoint();

        println("👀 Reading Dataset")
        val model = load_model(spark_context, model_path);

        println("👏👏 Recommend for the user ")
        recommend_to_user(
        spark_context, 
        model, movie_title, recommend_type, input_id);

        spark_context.stop();
    }
    //  Read Data from the Data path 
    def prepareData(sc: SparkContext, dataPath:String): RDD[(Int, String)] ={
        println("🧑‍🍳 Preparing Data")
        // reads data from dataPath into Spark RDD.
        val itemRDD: RDD[String] = sc.textFile(dataPath)
        // only takes in first two fields (movieID, movieName).
        val movieTitle: RDD[(Int, String)] = itemRDD.map(line => line.split("\\|")).map(x => (x(0).toInt, x(1)))
        // return movieID->movieName map as Spark RDD
        movieTitle
    }
    def load_model(spark_context: SparkContext, model_path: String): Option[MatrixFactorizationModel] = { 
        println("🎥 Loading model")
        try { 
            val model: MatrixFactorizationModel  = MatrixFactorizationModel.load(spark_context, model_path);
            Some(model)
        } catch { 
            case e: Exception => {
            println(e);
            None
        }
        } finally { 
            
        }
    }
    
    def recommend_to_user(
    spark: SparkContext, 
    model: Option[MatrixFactorizationModel], movieTitle:RDD[(Int, String)], arg1: String, arg2: String)={
        if (arg1 == "--USER") {
            recommendMovies(
            spark, 
            model.get, movieTitle, arg2.toInt)
        }
        if (arg1 == "--MOVIE") {
            recommendUsers(model.get, movieTitle, arg2.toInt)
        }
    }

    def recommendMovies(
        spark: SparkContext,
        model: MatrixFactorizationModel, 
        movieTitle: RDD[(Int, String)], 
        inputUserID: Int
        ) = {
        val recommendP = model.recommendProducts(inputUserID, 10)
        recommendP
        .foreach(p => {
            println("Inserting the values into the database")
            println(s"user: ${p.user}, recommended movie: ${movieTitle.lookup(p.product).mkString}, rating: ${p.rating}")
            // // val now = DateTime.now();
            val data = spark.cassandraTable("homepage_keyspace", "user_rating"); 
        });
    }

    def recommendUsers(model: MatrixFactorizationModel, movieTitle: RDD[(Int, String)], inputMovieID: Int) = {
        val recommendU = model.recommendUsers(inputMovieID, 110)
        println(s"Recommending the following users for movie ${inputMovieID.toString}:")
        recommendU.foreach(u => println(s"movie: ${movieTitle.lookup(u.product).mkString}, recommended user: ${u.user}, rating: ${u.rating}"))
    }
}
