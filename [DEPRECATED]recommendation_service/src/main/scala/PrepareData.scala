import org.apache.log4j.{Level, Logger}
import org.apache.spark.rdd.RDD
import org.apache.spark.{SparkConf, SparkContext}
import org.apache.spark.mllib.recommendation.{ALS, MatrixFactorizationModel, Rating}

//  Cleans up and Prepares the Data before REcommending movies 
object PrepareData  {
    def main(args: Array[String]): Unit = { 
        val spark_configuration = new SparkConf()
            .setAppName("CollaborativeFilter")
            .setMaster("local");
        val spark_context = new SparkContext(spark_configuration);
        println("🧑‍🔧 Setting up MasterNode " + spark_context.master)

        //  Running Logger
        println("🛰️ Running Logger")
        Logger.getLogger("org").setLevel(Level.ERROR);
        Logger.getLogger("akka").setLevel(Level.ERROR);

        println("🏗️ Loading Data");
        val data_path = "/home/philip/rust-programming/fullstack/Netflix-Clone-WASM/recommendation_service/ml-100k/u.data";
        val model_path = "/home/philip/rust-programming/fullstack/Netflix-Clone-WASM/recommendation_service/ml-100k/ALSmodel";
        val checkpoint_path = "/home/philip/rust-programming/fullstack/Netflix-Clone-WASM/recommendation_service/ml-100k/checkpoint/";
        spark_context.setCheckpointDir(checkpoint_path)
        

         println("👀Preparing Data")
        val ratingsRDD: RDD[Rating] = PrepareData(spark_context, data_path)
        ratingsRDD.checkpoint() // checkpoint data to avoid stackoverflow error

        println("👏Training")
        println("Start ALS training, rank=5, iteration=20, lambda=0.1")
        val model: MatrixFactorizationModel = ALS.train(ratingsRDD, 5, 20, 0.1)

        println("🧑‍🍳Saving Model")
        saveModel(spark_context, model, model_path)
        spark_context.stop()
    }
    def PrepareData(sc: SparkContext, dataPath:String): RDD[Rating] ={
        // reads data from dataPath into Spark RDD.
        val file: RDD[String] = sc.textFile(dataPath)
        // only takes in first three fields (userID, itemID, rating).
        val ratingsRDD: RDD[Rating] = file.map(line => line.split("\t") match {
        case Array(user, item, rate, _) => Rating(user.toInt, item.toInt, rate.toDouble)
        })
        println(ratingsRDD.first()) // Rating(196,242,3.0)
        // return processed data as Spark RDD
        ratingsRDD
  }

    def saveModel(context: SparkContext, model:MatrixFactorizationModel, modelPath: String): Unit ={
        try {
            model.save(context, modelPath)
        }   catch {
            case e: Exception => {
                println(e);
                println("Error Happened when saving model!!!")}
        }
            finally {
        }
    }
}
